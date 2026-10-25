use crate::catalog::Catalog;
use sillage::model::PlaybackState;
use sillage::playback::{Playback, PlaybackCommand, PlaybackEvent, PlaybackEventKind};
use slint::ComponentHandle;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, mpsc};
use std::time::Duration;

slint::include_modules!();

pub(crate) fn command(ui: &AppWindow, playback: &Playback, command: PlaybackCommand) -> bool {
    let loading = matches!(
        &command,
        PlaybackCommand::OpenLocal(_) | PlaybackCommand::OpenRemote(_)
    );
    ui.set_error_message("".into());
    if let Err(error) = playback.send(command) {
        ui.set_error_message(error.to_string().into());
        if loading {
            ui.set_busy(false);
        }
        return false;
    } else if loading {
        ui.set_busy(true);
        ui.set_status("Préparation du morceau…".into());
    }
    true
}

fn apply(ui: &AppWindow, event: PlaybackEvent, current: &AtomicU64, ended: &AtomicU64) {
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
            if matches!(state, PlaybackState::Paused | PlaybackState::Stopped) {
                ui.invoke_flush_state();
            }
            if matches!(state, PlaybackState::Stopped | PlaybackState::Empty) {
                ui.set_position_seconds(0.0);
                ui.set_elapsed_label("0:00".into());
            }
        }
        PlaybackEventKind::Loaded {
            title,
            duration,
            local,
        } => {
            if local {
                ui.set_selected_id("".into());
                ui.set_track_artist("Fichier local".into());
                ui.set_track_artwork(slint::Image::default());
                ui.invoke_clear_queue();
            }
            ui.set_track_title(title.into());
            ui.set_duration_seconds(
                duration
                    .map(|duration| duration.as_secs_f32())
                    .unwrap_or(0.0),
            );
            ui.set_position_seconds(0.0);
            ui.set_elapsed_label("0:00".into());
            let label = match duration {
                Some(duration) => {
                    let seconds = duration.as_secs();
                    format!("{}:{:02}", seconds / 60, seconds % 60)
                }
                None => String::new(),
            };
            ui.set_duration_label(label.into());
            ui.set_loaded(true);
            ui.set_error_message("".into());
        }
        PlaybackEventKind::VolumeChanged(_) => {}
        PlaybackEventKind::TrackMetadata(track) => {
            ui.invoke_track_metadata(crate::view::row(&track))
        }
        PlaybackEventKind::Ended => {
            if ended.swap(event.generation + 1, Ordering::AcqRel) == event.generation + 1 {
                return;
            }
            ui.set_playing(false);
            ui.set_status(PlaybackState::Ended.label().into());
            ui.invoke_ended();
        }
        PlaybackEventKind::Position(position) => {
            if !ui.get_busy() {
                ui.invoke_audio_position(position.as_millis() as f32);
            }
            if !ui.get_busy() && ui.window().is_visible() && !ui.window().is_minimized() {
                ui.set_position_seconds(position.as_secs_f32());
                ui.set_elapsed_label(
                    format!("{}:{:02}", position.as_secs() / 60, position.as_secs() % 60).into(),
                );
            }
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
    let search = Arc::new(Catalog::new(ui.as_weak())?);
    let artwork = Arc::new(crate::artwork::Artwork::new(ui.as_weak())?);
    let session = crate::session::wire(&ui, playback.clone(), artwork.clone())?;
    for force in [false, true] {
        let weak = ui.as_weak();
        let catalog = search.clone();
        let load = move || {
            if let Some(ui) = weak.upgrade() {
                ui.set_home_loading(true);
                ui.set_home_error("".into());
                if let Err(message) = catalog.home(force) {
                    ui.set_home_loading(false);
                    ui.set_home_error(message.into());
                }
            }
        };
        if force {
            ui.on_refresh_home(load);
        } else {
            ui.on_load_home(load);
        }
    }
    let debounce = Rc::new(slint::Timer::default());
    {
        let weak = ui.as_weak();
        let search = search.clone();
        let debounce = debounce.clone();
        ui.on_query_edited(move |query| {
            search.cancel();
            // Switching to search cancels an in-flight home request without hiding cached content.
            debounce.stop();
            let Some(ui) = weak.upgrade() else {
                return;
            };
            ui.set_results(slint::ModelRc::default());
            ui.set_searched(false);
            ui.set_searching(false);
            ui.set_search_error("".into());
            ui.set_home_loading(false);
            if query.trim().is_empty() {
                return;
            }
            let weak = weak.clone();
            let search = search.clone();
            debounce.start(
                slint::TimerMode::SingleShot,
                Duration::from_millis(200),
                move || {
                    if let Some(ui) = weak.upgrade() {
                        submit_search(&ui, &search);
                    }
                },
            );
        });
    }
    {
        let weak = ui.as_weak();
        let search = search.clone();
        let debounce = debounce.clone();
        ui.on_search(move || {
            debounce.stop();
            if let Some(ui) = weak.upgrade() {
                submit_search(&ui, &search);
            }
        });
    }
    let about: Rc<RefCell<Option<AboutWindow>>> = Rc::new(RefCell::new(None));
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
                            ui.set_error_message(
                                "La fenêtre À propos ne peut pas être ouverte.".into(),
                            );
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
    let ended = Arc::new(AtomicU64::new(0));
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
                    let ended = ended.clone();
                    if weak
                        .upgrade_in_event_loop(move |ui| apply(&ui, event, &current, &ended))
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
    ui.invoke_load_home();
    let result = ui.run();
    let _ = start.send(());
    session.flush();
    drop(ui);
    drop(about);
    drop(playback);
    drop(debounce);
    drop(search);
    drop(session);
    drop(artwork);
    let _ = bridge.join();
    result.map_err(Into::into)
}

fn submit_search(ui: &AppWindow, search: &Catalog) {
    ui.set_results(slint::ModelRc::default());
    ui.set_searched(false);
    match search.search(ui.get_query().to_string()) {
        Ok(()) => {
            ui.set_searching(true);
            ui.set_search_error("".into());
        }
        Err(message) => {
            ui.set_searching(false);
            ui.set_search_error(message.into());
        }
    }
}
