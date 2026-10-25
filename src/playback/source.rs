use rodio::{Decoder, Source};
use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::thread::JoinHandle;
use std::time::Duration;

const CHUNK_FRAMES: usize = 1024;

pub(super) struct Prepared {
    pub source: Box<dyn Source + Send>,
    pub worker: JoinHandle<()>,
    pub failed: Arc<AtomicBool>,
}

pub(super) fn prepare(path: &Path) -> Result<Prepared, &'static str> {
    let file =
        File::open(path).map_err(|_| "This file could not be opened. Choose another file.")?;
    if !file
        .metadata()
        .map_err(|_| "The file is unavailable.")?
        .is_file()
    {
        return Err("Choose a regular audio file.");
    }
    let decoder = Decoder::try_from(file)
        .map_err(|_| "This file could not be decoded. Try MP3, M4A, WAV, FLAC or Ogg Vorbis.")?;
    prepare_decoder(Box::new(decoder), Arc::new(AtomicBool::new(false)))
}

pub(super) fn prepare_online(
    client: &crate::service::Client,
    stream: crate::service::ResolvedStream,
    cancel: crate::service::Cancellation,
) -> Result<Prepared, &'static str> {
    let reader = client.input(stream, cancel);
    let failed = reader.failure_flag();
    let decoder = Decoder::builder()
        .with_data(reader)
        .with_hint("m4a")
        .with_seekable(false)
        .build()
        .map_err(|_| "The audio stream could not be decoded. Try another track.")?;
    prepare_decoder(Box::new(decoder), failed)
}

fn prepare_decoder(
    mut decoder: Box<dyn Source + Send>,
    failed: Arc<AtomicBool>,
) -> Result<Prepared, &'static str> {
    let channels = decoder.channels();
    let rate = decoder.sample_rate();
    if channels.get() > 8 || rate.get() > 192_000 {
        return Err("This audio format exceeds the local player's supported dimensions.");
    }
    let duration = decoder.total_duration();
    let decoder_failed = failed.clone();
    let (sender, receiver) = mpsc::sync_channel(16);
    let worker = std::thread::Builder::new()
        .name("sillage-decode".into())
        .spawn(move || {
            loop {
                let samples: Vec<_> = decoder
                    .by_ref()
                    .take(CHUNK_FRAMES * usize::from(channels.get()))
                    .collect();
                if decoder.channels() != channels
                    || decoder.sample_rate() != rate
                    || samples.iter().any(|sample| !sample.is_finite())
                {
                    decoder_failed.store(true, Ordering::Release);
                    return;
                }
                if samples.is_empty() || sender.send(samples).is_err() {
                    return;
                }
            }
        })
        .map_err(|_| "The decoder worker could not start. Try again.")?;
    Ok(Prepared {
        source: Box::new(PcmSource {
            receiver,
            samples: Vec::new(),
            at: 0,
            silence: 0,
            channels,
            rate,
            duration,
        }),
        worker,
        failed,
    })
}

struct PcmSource {
    receiver: Receiver<Vec<rodio::Sample>>,
    samples: Vec<rodio::Sample>,
    at: usize,
    silence: usize,
    channels: rodio::ChannelCount,
    rate: rodio::SampleRate,
    duration: Option<Duration>,
}

impl Iterator for PcmSource {
    type Item = rodio::Sample;
    fn next(&mut self) -> Option<Self::Item> {
        if self.silence > 0 {
            self.silence -= 1;
            return Some(0.0);
        }
        if self.at == self.samples.len() {
            match self.receiver.try_recv() {
                Ok(samples) => {
                    self.samples = samples;
                    self.at = 0;
                }
                Err(TryRecvError::Disconnected) => return None,
                Err(TryRecvError::Empty) => {
                    // Keep file I/O and decoder stalls off the real-time callback.
                    self.silence = usize::from(self.channels.get()) * 256 - 1;
                    return Some(0.0);
                }
            }
        }
        let sample = self.samples[self.at];
        self.at += 1;
        Some(sample)
    }
}

impl Source for PcmSource {
    fn current_span_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> rodio::ChannelCount {
        self.channels
    }
    fn sample_rate(&self) -> rodio::SampleRate {
        self.rate
    }
    fn total_duration(&self) -> Option<Duration> {
        self.duration
    }
}
