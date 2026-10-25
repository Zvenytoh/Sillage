use crate::model::PlaybackState;
mod source;

use rodio::{DeviceSinkBuilder, MixerDeviceSink, Player, Source};
use std::fmt;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::{self, SyncSender};
use std::thread::JoinHandle;
use std::time::Duration;

pub enum PlaybackCommand {
    OpenLocal(PathBuf),
    Play,
    Pause,
    Stop,
    SetVolume(f32),
    Seek(Duration),
}

pub struct PlaybackEvent {
    pub generation: u64,
    pub kind: PlaybackEventKind,
}

pub enum PlaybackEventKind {
    Ready,
    StateChanged(PlaybackState),
    Loaded {
        title: String,
        duration: Option<Duration>,
    },
    VolumeChanged(f32),
    Ended,
    Failed(&'static str),
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlaybackError {
    InvalidVolume,
    Busy,
    Unavailable,
}

impl fmt::Display for PlaybackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidVolume => "Volume must be finite and between zero and one.",
            Self::Busy => "The player is busy. Try again.",
            Self::Unavailable => "The audio worker is unavailable. Restart Sillage.",
        })
    }
}

impl std::error::Error for PlaybackError {}

pub struct Playback {
    sender: Option<SyncSender<PlaybackCommand>>,
    shutdown: Arc<AtomicBool>,
    worker: Option<JoinHandle<()>>,
    callbacks: Arc<CallbackState>,
}

#[derive(Default)]
struct CallbackState {
    ended: AtomicU64,
    failed: AtomicU64,
    device_failed: AtomicBool,
}

pub struct CallbackEvents(Arc<CallbackState>);

impl Playback {
    pub fn spawn(events: SyncSender<PlaybackEvent>) -> Result<Self, PlaybackError> {
        let (sender, receiver) = mpsc::sync_channel(32);
        let shutdown = Arc::new(AtomicBool::new(false));
        let stopping = shutdown.clone();
        let callbacks = Arc::new(CallbackState::default());
        let worker_callbacks = callbacks.clone();
        let worker = std::thread::Builder::new()
            .name("sillage-audio".into())
            .spawn(move || {
                let mut audio = Worker::new(events, worker_callbacks);
                audio.emit(PlaybackEventKind::Ready);
                while let Ok(command) = receiver.recv() {
                    if stopping.load(Ordering::Acquire) {
                        break;
                    }
                    audio.apply(command);
                }
            })
            .map_err(|_| PlaybackError::Unavailable)?;
        Ok(Self {
            sender: Some(sender),
            shutdown,
            worker: Some(worker),
            callbacks,
        })
    }

    pub fn callback_events(&self) -> CallbackEvents {
        CallbackEvents(self.callbacks.clone())
    }

    pub fn send(&self, command: PlaybackCommand) -> Result<(), PlaybackError> {
        if let PlaybackCommand::SetVolume(value) = &command
            && (!value.is_finite() || !(0.0..=1.0).contains(value))
        {
            return Err(PlaybackError::InvalidVolume);
        }
        self.sender
            .as_ref()
            .ok_or(PlaybackError::Unavailable)?
            .try_send(command)
            .map_err(|error| match error {
                mpsc::TrySendError::Full(_) => PlaybackError::Busy,
                mpsc::TrySendError::Disconnected(_) => PlaybackError::Unavailable,
            })
    }
}

impl CallbackEvents {
    pub fn drain(&self) -> Vec<PlaybackEvent> {
        let mut events = Vec::with_capacity(2);
        for (slot, kind) in [
            (&self.0.ended, PlaybackEventKind::Ended),
            (
                &self.0.failed,
                PlaybackEventKind::Failed("Audio output failed. Stop playback and try again."),
            ),
        ] {
            let encoded = slot.swap(0, Ordering::AcqRel);
            if encoded != 0 {
                events.push(PlaybackEvent {
                    generation: encoded - 1,
                    kind,
                });
            }
        }
        events
    }
}

impl Drop for Playback {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::Release);
        self.sender.take();
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

type AudioSource = Box<dyn Source + Send>;

struct Worker {
    events: SyncSender<PlaybackEvent>,
    generation: Arc<AtomicU64>,
    path: Option<PathBuf>,
    pending: Option<AudioSource>,
    output: Option<(MixerDeviceSink, Player)>,
    state: PlaybackState,
    volume: f32,
    callbacks: Arc<CallbackState>,
    decoder: Option<JoinHandle<()>>,
    source_failed: Arc<AtomicBool>,
}

impl Worker {
    fn new(events: SyncSender<PlaybackEvent>, callbacks: Arc<CallbackState>) -> Self {
        Self {
            events,
            generation: Arc::new(AtomicU64::new(0)),
            path: None,
            pending: None,
            output: None,
            state: PlaybackState::Empty,
            volume: 0.25,
            callbacks,
            decoder: None,
            source_failed: Arc::new(AtomicBool::new(false)),
        }
    }

    fn emit(&self, kind: PlaybackEventKind) {
        // Backpressure is confined to the worker, never the UI or audio callback.
        let _ = self.events.send(PlaybackEvent {
            generation: self.generation.load(Ordering::Acquire),
            kind,
        });
    }

    fn set_state(&mut self, state: PlaybackState) {
        self.state = state;
        self.emit(PlaybackEventKind::StateChanged(state));
    }

    fn clear_source(&mut self) {
        self.output = None;
        self.pending = None;
        if let Some(decoder) = self.decoder.take() {
            let _ = decoder.join();
        }
    }

    fn install(&mut self, prepared: source::Prepared) {
        self.clear_source();
        self.pending = Some(prepared.source);
        self.decoder = Some(prepared.worker);
        self.source_failed = prepared.failed;
    }

    fn apply(&mut self, command: PlaybackCommand) {
        match command {
            PlaybackCommand::OpenLocal(path) => {
                let previous = self.state;
                self.set_state(PlaybackState::Loading);
                match source::prepare(&path) {
                    Ok(prepared) => {
                        let duration = prepared.source.total_duration();
                        let title = path
                            .file_stem()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .into_owned();
                        self.generation.fetch_add(1, Ordering::AcqRel);
                        self.install(prepared);
                        self.path = Some(path);
                        self.emit(PlaybackEventKind::Loaded { title, duration });
                        self.set_state(PlaybackState::Paused);
                    }
                    Err(message) => {
                        let restore = if previous == PlaybackState::Playing
                            && self
                                .output
                                .as_ref()
                                .is_some_and(|(_, player)| player.empty())
                        {
                            PlaybackState::Ended
                        } else {
                            previous
                        };
                        self.emit(PlaybackEventKind::Failed(message));
                        self.set_state(restore);
                    }
                }
            }
            PlaybackCommand::Play => {
                if self.callbacks.device_failed.swap(false, Ordering::AcqRel) {
                    self.clear_source();
                }
                if let Some((_, player)) = &self.output
                    && !player.empty()
                {
                    player.play();
                    self.set_state(PlaybackState::Playing);
                    return;
                }
                if self.pending.is_none() {
                    let Some(path) = &self.path else {
                        return;
                    };
                    match source::prepare(path) {
                        Ok(prepared) => self.install(prepared),
                        Err(message) => {
                            self.emit(PlaybackEventKind::Failed(message));
                            return;
                        }
                    }
                }
                let Some(source) = self.pending.take() else {
                    return;
                };
                let events = self.events.clone();
                let generation = self.generation.clone();
                let id = generation.fetch_add(1, Ordering::AcqRel) + 1;
                let error_generation = generation.clone();
                let callbacks = self.callbacks.clone();
                let output = DeviceSinkBuilder::from_default_device().and_then(|builder| {
                    builder
                        .with_error_callback(move |_| {
                            if error_generation.load(Ordering::Acquire) != id {
                                return;
                            }
                            callbacks.device_failed.store(true, Ordering::Release);
                            callbacks.failed.store(id + 1, Ordering::Release);
                            let _ = events.try_send(PlaybackEvent {
                                generation: id,
                                kind: PlaybackEventKind::Failed(
                                    "Audio processing failed. Stop playback and try again.",
                                ),
                            });
                        })
                        .open_sink_or_fallback()
                });
                match output {
                    Ok(mut output) => {
                        output.log_on_drop(false);
                        let player = Player::connect_new(output.mixer());
                        player.pause();
                        player.set_volume(self.volume);
                        player.append(EndNotification {
                            inner: source,
                            events: self.events.clone(),
                            generation,
                            id,
                            notified: false,
                            callbacks: self.callbacks.clone(),
                            failed: self.source_failed.clone(),
                        });
                        self.output = Some((output, player));
                        self.set_state(PlaybackState::Playing);
                        if let Some((_, player)) = &self.output {
                            player.play();
                        }
                    }
                    Err(_) => self.emit(PlaybackEventKind::Failed(
                        "No usable default audio output. Check your device and press Play again.",
                    )),
                }
            }
            PlaybackCommand::Pause => {
                if let Some((_, player)) = &self.output {
                    player.pause();
                    self.set_state(PlaybackState::Paused);
                }
            }
            PlaybackCommand::Stop => {
                self.generation.fetch_add(1, Ordering::AcqRel);
                self.clear_source();
                self.set_state(if self.path.is_some() {
                    PlaybackState::Stopped
                } else {
                    PlaybackState::Empty
                });
            }
            PlaybackCommand::SetVolume(value) => {
                self.volume = value;
                if let Some((_, player)) = &self.output {
                    player.set_volume(value);
                }
                self.emit(PlaybackEventKind::VolumeChanged(value));
            }
            PlaybackCommand::Seek(position) => {
                if let Some((_, player)) = &self.output
                    && player.try_seek(position).is_err()
                {
                    self.emit(PlaybackEventKind::Failed(
                        "Seeking is unavailable for this file.",
                    ));
                }
            }
        }
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        self.clear_source();
    }
}

struct EndNotification {
    inner: AudioSource,
    events: SyncSender<PlaybackEvent>,
    generation: Arc<AtomicU64>,
    id: u64,
    notified: bool,
    callbacks: Arc<CallbackState>,
    failed: Arc<AtomicBool>,
}

impl Iterator for EndNotification {
    type Item = rodio::Sample;
    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.inner.next();
        if sample.is_none() && !self.notified {
            self.notified = true;
            if self.generation.load(Ordering::Acquire) == self.id {
                if self.failed.load(Ordering::Acquire) {
                    self.callbacks.failed.store(self.id + 1, Ordering::Release);
                    let _ = self.events.try_send(PlaybackEvent {
                        generation: self.id,
                        kind: PlaybackEventKind::Failed(
                            "Audio decoding failed. Stop playback and choose another file.",
                        ),
                    });
                    return sample;
                }
                // A full wake-up channel already contains work for the bridge.
                self.callbacks.ended.store(self.id + 1, Ordering::Release);
                let _ = self.events.try_send(PlaybackEvent {
                    generation: self.id,
                    kind: PlaybackEventKind::Ended,
                });
            }
        }
        sample
    }
}

impl Source for EndNotification {
    fn current_span_len(&self) -> Option<usize> {
        self.inner.current_span_len()
    }
    fn channels(&self) -> rodio::ChannelCount {
        self.inner.channels()
    }
    fn sample_rate(&self) -> rodio::SampleRate {
        self.inner.sample_rate()
    }
    fn total_duration(&self) -> Option<Duration> {
        self.inner.total_duration()
    }
    fn try_seek(&mut self, pos: Duration) -> Result<(), rodio::source::SeekError> {
        self.inner.try_seek(pos)
    }
}
