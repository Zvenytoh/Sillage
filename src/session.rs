use crate::app::{AppWindow, SearchRow, command};
use crate::library::Library;
use sillage::model::Track;
use sillage::playback::{Playback, PlaybackCommand};
use slint::{ComponentHandle, Model};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::hash::{BuildHasher, RandomState};
use std::rc::Rc;
use std::sync::Arc;

#[derive(Default)]
struct Session {
    queue: Vec<SearchRow>,
    current: Option<usize>,
    history: VecDeque<usize>,
    random: RandomState,
    counter: u64,
    volume: f32,
}

fn rows(model: slint::ModelRc<SearchRow>) -> Vec<SearchRow> {
    model.iter().take(500).collect()
}

fn limit_covers(rows: &mut [SearchRow]) {
    let mut retained = 0;
    for row in rows.iter_mut().rev() {
        if row.artwork.size().width > 0 {
            retained += 1;
            if retained > 24 {
                row.artwork = slint::Image::default();
            }
        }
    }
}

fn refresh(ui: &AppWindow, session: &Session) {
    ui.set_queue_rows(slint::ModelRc::new(slint::VecModel::from(
        session.queue.clone(),
    )));
    ui.set_can_next(
        session
            .current
            .is_some_and(|index| index + 1 < session.queue.len())
            || (ui.get_shuffle() && session.queue.len() > 1),
    );
    ui.set_can_previous(
        !session.history.is_empty() || session.current.is_some_and(|index| index > 0),
    );
    ui.set_saved_current(
        ui.get_saved_rows()
            .iter()
            .any(|row| row.id == ui.get_selected_id()),
    );
}

fn start(ui: &AppWindow, playback: &Playback, row: SearchRow) -> bool {
    let track = Track {
        id: row.id.to_string(),
        title: row.title.to_string(),
        artists: vec![row.artist.to_string()],
        duration_ms: None,
        thumbnail: None,
    };
    if !command(ui, playback, PlaybackCommand::OpenRemote(track)) {
        return false;
    }
    ui.set_selected_id(row.id.clone());
    ui.set_track_title(row.title.clone());
    ui.set_track_artist(row.artist.clone());
    ui.set_track_artwork(row.artwork.clone());
    ui.set_details_open(true);
    ui.set_position_seconds(0.0);
    ui.set_elapsed_label("0:00".into());
    true
}

fn remember(session: &mut Session, current: usize) {
    if session.history.len() == 128 {
        session.history.pop_front();
    }
    session.history.push_back(current);
}

fn save_row(ui: &AppWindow, library: &Library, row: SearchRow) {
    let mut saved = rows(ui.get_saved_rows());
    if let Some(index) = saved.iter().position(|saved| saved.id == row.id) {
        saved.remove(index);
    } else if saved.len() < 500 {
        saved.push(row);
    } else {
        ui.set_error_message(
            "Your local library holds up to 500 songs. Remove one before saving another.".into(),
        );
        return;
    }
    limit_covers(&mut saved);
    match library.save(&saved) {
        Ok(()) => {
            ui.set_saved_rows(slint::ModelRc::new(slint::VecModel::from(saved)));
            ui.set_saved_current(
                ui.get_saved_rows()
                    .iter()
                    .any(|row| row.id == ui.get_selected_id()),
            );
        }
        Err(message) => ui.set_error_message(message.into()),
    }
}

pub(crate) fn wire(ui: &AppWindow, playback: Arc<Playback>) -> std::io::Result<Arc<Library>> {
    let library = Arc::new(Library::spawn(ui.as_weak())?);
    let state = Rc::new(RefCell::new(Session {
        volume: 25.0,
        ..Session::default()
    }));
    {
        let weak = ui.as_weak();
        let playback = playback.clone();
        let state = state.clone();
        ui.on_play_track(move |row| {
            let Some(ui) = weak.upgrade() else {
                return;
            };
            if !start(&ui, &playback, row.clone()) {
                return;
            }
            let candidates = rows(if ui.get_page() == 2 {
                ui.get_saved_rows()
            } else {
                ui.get_results()
            });
            let mut state = state.borrow_mut();
            state.queue = if candidates.iter().any(|item| item.id == row.id) {
                candidates
            } else {
                vec![row.clone()]
            };
            state.current = state.queue.iter().position(|item| item.id == row.id);
            state.history.clear();
            refresh(&ui, &state);
        });
    }
    {
        let weak = ui.as_weak();
        let state = state.clone();
        let playback = playback.clone();
        ui.on_play_queued(move |row| {
            if let Some(ui) = weak.upgrade()
                && start(&ui, &playback, row.clone())
            {
                let mut state = state.borrow_mut();
                if let Some(current) = state.current {
                    remember(&mut state, current);
                }
                state.current = state.queue.iter().position(|item| item.id == row.id);
                refresh(&ui, &state);
            }
        });
    }
    for ended in [false, true] {
        let weak = ui.as_weak();
        let state = state.clone();
        let playback = playback.clone();
        let advance = move || {
            let Some(ui) = weak.upgrade() else {
                return;
            };
            let mut state = state.borrow_mut();
            let Some(current) = state.current else {
                return;
            };
            if state.queue.is_empty() {
                return;
            }
            let next = if ended && ui.get_repeat() {
                current
            } else if ui.get_shuffle() && state.queue.len() > 1 {
                state.counter = state.counter.wrapping_add(1);
                let offset =
                    state.random.hash_one(state.counter) as usize % (state.queue.len() - 1) + 1;
                (current + offset) % state.queue.len()
            } else {
                current + 1
            };
            if let Some(row) = state.queue.get(next).cloned()
                && start(&ui, &playback, row)
            {
                if next != current {
                    remember(&mut state, current);
                }
                state.current = Some(next);
            }
            refresh(&ui, &state);
        };
        if ended {
            ui.on_ended(advance);
        } else {
            ui.on_next_track(advance);
        }
    }
    {
        let weak = ui.as_weak();
        let state = state.clone();
        let playback = playback.clone();
        ui.on_previous_track(move || {
            let Some(ui) = weak.upgrade() else {
                return;
            };
            let mut state = state.borrow_mut();
            if let Some(index) = state
                .history
                .back()
                .copied()
                .or_else(|| state.current.and_then(|index| index.checked_sub(1)))
                && let Some(row) = state.queue.get(index).cloned()
                && start(&ui, &playback, row)
            {
                if state.history.back() == Some(&index) {
                    state.history.pop_back();
                }
                state.current = Some(index);
            }
            refresh(&ui, &state);
        });
    }
    {
        let weak = ui.as_weak();
        let state = state.clone();
        ui.on_enqueue(move |row| {
            let Some(ui) = weak.upgrade() else {
                return;
            };
            let mut state = state.borrow_mut();
            if state.queue.len() >= 500 {
                ui.set_error_message("The session queue holds up to 500 songs.".into());
                return;
            }
            if !state.queue.iter().any(|item| item.id == row.id) {
                state.queue.push(row);
            }
            limit_covers(&mut state.queue);
            refresh(&ui, &state);
            ui.set_queue_open(true);
        });
    }
    {
        let weak = ui.as_weak();
        let state = state.clone();
        ui.on_queue_mode_changed(move || {
            if let Some(ui) = weak.upgrade() {
                refresh(&ui, &state.borrow());
            }
        });
    }
    {
        let weak = ui.as_weak();
        let state = state.clone();
        ui.on_artwork_ready(move |loaded| {
            let Some(ui) = weak.upgrade() else {
                return;
            };
            let mut state = state.borrow_mut();
            for row in &mut state.queue {
                if row.id == loaded.id {
                    row.artwork = loaded.artwork.clone();
                }
            }
            limit_covers(&mut state.queue);
            let mut saved = rows(ui.get_saved_rows());
            let mut changed = false;
            for row in &mut saved {
                if row.id == loaded.id {
                    row.artwork = loaded.artwork.clone();
                    changed = true;
                }
            }
            if changed {
                limit_covers(&mut saved);
                ui.set_saved_rows(slint::ModelRc::new(slint::VecModel::from(saved)));
            }
            refresh(&ui, &state);
        });
    }
    {
        let weak = ui.as_weak();
        let state = state.clone();
        ui.on_clear_queue(move || {
            if let Some(ui) = weak.upgrade() {
                let mut state = state.borrow_mut();
                state.queue.clear();
                state.current = None;
                state.history.clear();
                refresh(&ui, &state);
            }
        });
    }
    {
        let weak = ui.as_weak();
        let library = library.clone();
        ui.on_save_track(move |row| {
            if let Some(ui) = weak.upgrade() {
                save_row(&ui, &library, row);
            }
        });
    }
    {
        let weak = ui.as_weak();
        let library = library.clone();
        ui.on_save_current(move || {
            if let Some(ui) = weak.upgrade()
                && !ui.get_selected_id().is_empty()
            {
                let row = SearchRow {
                    id: ui.get_selected_id(),
                    title: ui.get_track_title(),
                    artist: ui.get_track_artist(),
                    artwork: ui.get_track_artwork(),
                    duration: ui.get_duration_label(),
                };
                save_row(&ui, &library, row);
            }
        });
    }
    {
        let weak = ui.as_weak();
        let state = state.clone();
        ui.on_mute(move || {
            if let Some(ui) = weak.upgrade() {
                let mut state = state.borrow_mut();
                let volume = if ui.get_volume() > 0.0 {
                    state.volume = ui.get_volume();
                    0.0
                } else {
                    state.volume.max(1.0)
                };
                command(&ui, &playback, PlaybackCommand::SetVolume(volume / 100.0));
            }
        });
    }
    Ok(library)
}
