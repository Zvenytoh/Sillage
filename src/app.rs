use crate::platform::file_dialog;
use sillage::model::PlaybackState;
use sillage::playback::{Playback, PlaybackCommand, PlaybackEvent, PlaybackEventKind};
use slint::ComponentHandle;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, mpsc};
use std::time::Duration;

slint::include_modules!();

fn command(ui: &AppWindow, playback: &Playback, command: PlaybackCommand) {
    ui.set_error_message("".into());
    if let Err(error) = playback.send(command) {
        ui.set_error_message(error.to_string().into());
    }
}

fn apply(ui: &AppWindow, event: PlaybackEvent, current: &AtomicU64) {
    if event.generation < current.load(Ordering::Acquire) {
        return;
    }
    current.store(event.generation, Ordering::Release);
    match event.kind {
        PlaybackEventKind::Ready => {}
        PlaybackEventKind::StateChanged(state) => {
            ui.set_status(state.label().into());
            ui.set_playing(state == PlaybackState::Playing);
            ui.set_busy(state == PlaybackState::Loading);
        }
        PlaybackEventKind::Loaded { title, duration } => {
            ui.set_track_title(title.into());
            let label = match duration {
                Some(duration) => {
                    let seconds = duration.as_secs();
                    format!("{}:{:02} · Local audio", seconds / 60, seconds % 60)
                }
                None => "Local audio · Length unavailable".into(),
            };
            ui.set_duration_label(label.into());
            ui.set_loaded(true);
            ui.set_error_message("".into());
        }
        PlaybackEventKind::VolumeChanged(value) => ui.set_volume(value * 100.0),
        PlaybackEventKind::Ended => {
            ui.set_playing(false);
            ui.set_status(PlaybackState::Ended.label().into());
        }
        PlaybackEventKind::Failed(message) => {
            ui.set_error_message(message.into());
            ui.set_busy(false);
        }
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let ui = AppWindow::new()?;
    let (events, receiver) = mpsc::sync_channel(32);
    let playback = Arc::new(Playback::spawn(events)?);
    let about: Rc<RefCell<Option<AboutWindow>>> = Rc::new(RefCell::new(None));
    {
        let weak = ui.as_weak();
        let playback = playback.clone();
        ui.on_open_file(move || {
            let Some(ui) = weak.upgrade() else {
                return;
            };
            match file_dialog::open_audio() {
                Ok(Some(path)) => command(&ui, &playback, PlaybackCommand::OpenLocal(path)),
                Ok(None) => {}
                Err(message) => ui.set_error_message(message.into()),
            }
        });
    }
    {
        let weak = ui.as_weak();
        let playback = playback.clone();
        ui.on_toggle_play(move || {
            if let Some(ui) = weak.upgrade() {
                let action = if ui.get_playing() {
                    PlaybackCommand::Pause
                } else {
                    PlaybackCommand::Play
                };
                command(&ui, &playback, action);
            }
        });
    }
    {
        let weak = ui.as_weak();
        let playback = playback.clone();
        ui.on_stop(move || {
            if let Some(ui) = weak.upgrade() {
                command(&ui, &playback, PlaybackCommand::Stop);
            }
        });
    }
    {
        let weak = ui.as_weak();
        let playback = playback.clone();
        ui.on_volume_changed(move |value| {
            if let Some(ui) = weak.upgrade() {
                command(&ui, &playback, PlaybackCommand::SetVolume(value));
            }
        });
    }
    {
        let weak = ui.as_weak();
        let about = about.clone();
        ui.on_show_about(move || {
            if about.borrow().is_none() {
                match AboutWindow::new() {
                    Ok(window) => *about.borrow_mut() = Some(window),
                    Err(_) => {
                        if let Some(ui) = weak.upgrade() {
                            ui.set_error_message("The About window could not be opened.".into());
                        }
                        return;
                    }
                }
            }
            if let Some(window) = about.borrow().as_ref() {
                let _ = window.show();
            }
        });
    }
    let weak = ui.as_weak();
    let current = Arc::new(AtomicU64::new(0));
    let (start, started) = mpsc::channel();
    let callback_events = playback.callback_events();
    let bridge = std::thread::Builder::new()
        .name("sillage-events".into())
        .spawn(move || {
            if started.recv().is_err() {
                return;
            }
            while let Ok(event) = receiver.recv() {
                let mut events = vec![event];
                events.extend(callback_events.drain());
                for event in events {
                    let current = current.clone();
                    if weak
                        .upgrade_in_event_loop(move |ui| apply(&ui, event, &current))
                        .is_err()
                    {
                        return;
                    }
                }
            }
        })?;
    let start_on_timer = start.clone();
    slint::Timer::single_shot(Duration::ZERO, move || {
        let _ = start_on_timer.send(());
    });
    {
        let about = about.clone();
        ui.window().on_close_requested(move || {
            let window = about.borrow().as_ref().map(ComponentHandle::as_weak);
            if let Some(window) = window.and_then(|window| window.upgrade()) {
                let _ = window.hide();
            }
            slint::CloseRequestResponse::HideWindow
        });
    }
    let result = ui.run();
    let _ = start.send(());
    drop(ui);
    drop(about);
    drop(playback);
    let _ = bridge.join();
    result.map_err(Into::into)
}
