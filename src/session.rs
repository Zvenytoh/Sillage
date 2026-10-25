use crate::app::{AppWindow, SearchRow, command};
use crate::artwork::{Artwork, ArtworkPriority};
use crate::library::Library;
use sillage::history::{History, LibrarySnapshot};
use sillage::model::{Track, now_ms};
use sillage::playback::{Playback, PlaybackCommand};
use slint::{ComponentHandle, Model};
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::hash::{BuildHasher, RandomState};
use std::rc::Rc;
use std::sync::Arc;

#[derive(Default)]
struct State {
    queue: Vec<SearchRow>,
    cursor: Option<usize>,
    current: Option<SearchRow>,
    last_position_ms: u64,
    back: VecDeque<usize>,
    recent: History,
    favorites: Vec<Track>,
    library_loaded: bool,
    dirty: bool,
    images: VecDeque<SearchRow>,
    random: RandomState,
    counter: u64,
    volume: f32,
    navigation: VecDeque<i32>,
    navigation_cursor: usize,
}
struct Controller {
    ui: slint::Weak<AppWindow>,
    state: RefCell<State>,
    playback: Arc<Playback>,
    library: Arc<Library>,
    artwork: Arc<Artwork>,
}
pub(crate) struct SessionHandle(Rc<Controller>);
impl SessionHandle {
    pub fn flush(&self) {
        self.0.flush();
    }
}
fn rows(model: slint::ModelRc<SearchRow>) -> Vec<SearchRow> {
    model.iter().take(500).collect()
}

impl Controller {
    fn flush(&self) {
        let mut state = self.state.borrow_mut();
        if !state.library_loaded || !state.dirty {
            return;
        }
        let snapshot = LibrarySnapshot {
            favorites: state.favorites.clone(),
            recent: state.recent.entries.clone(),
        };
        match self.library.save(&snapshot) {
            Ok(()) => state.dirty = false,
            Err(message) => {
                if let Some(ui) = self.ui.upgrade() {
                    ui.set_error_message(message.into());
                }
            }
        }
    }
    fn refresh(&self) {
        let Some(ui) = self.ui.upgrade() else {
            return;
        };
        {
            let state = self.state.borrow();
            ui.set_queue_rows(slint::ModelRc::new(slint::VecModel::from(
                state.queue.clone(),
            )));
            ui.set_up_next_rows(slint::ModelRc::new(slint::VecModel::from(
                state
                    .queue
                    .iter()
                    .skip(state.cursor.map(|index| index + 1).unwrap_or(0))
                    .take(6)
                    .cloned()
                    .collect::<Vec<_>>(),
            )));
            ui.set_saved_rows(crate::view::model(&state.favorites));
            ui.set_recent_rows(crate::view::model(
                &state
                    .recent
                    .entries
                    .iter()
                    .map(|entry| entry.track.clone())
                    .collect::<Vec<_>>(),
            ));
            ui.set_can_next(
                state
                    .cursor
                    .is_some_and(|index| index + 1 < state.queue.len())
                    || (ui.get_shuffle() && state.queue.len() > 1),
            );
            ui.set_can_previous(
                !state.back.is_empty() || state.cursor.is_some_and(|index| index > 0),
            );
            ui.set_saved_current(
                state
                    .favorites
                    .iter()
                    .any(|track| track.id == ui.get_selected_id().as_str()),
            );
            for image in &state.images {
                crate::view::image(&ui, &image.id, &image.artwork);
            }
        }
        self.request_visible();
    }
    fn request(&self, mut row: SearchRow, priority: ArtworkPriority) {
        if row.artwork.size().width > 0 {
            return;
        }
        if row.id.is_empty() {
            return;
        }
        let Some(ui) = self.ui.upgrade() else {
            return;
        };
        if let Some(image) = self
            .state
            .borrow()
            .images
            .iter()
            .find(|image| image.id == row.id)
        {
            crate::view::image(&ui, &row.id, &image.artwork);
            return;
        }
        if row.id == ui.get_selected_id() && row.duration_ms == 0.0 {
            row.duration_ms = ui.get_duration_seconds() * 1000.0;
        }
        self.artwork.request(&crate::view::track(&row), priority);
    }
    fn request_visible(&self) {
        let Some(ui) = self.ui.upgrade() else {
            return;
        };
        let visible = match ui.get_page() {
            0 => ui
                .get_recent_rows()
                .iter()
                .take(6)
                .chain(ui.get_trending_rows().iter().take(8))
                .chain(ui.get_discovery_rows().iter().take(8))
                .collect::<Vec<_>>(),
            2 => ui.get_saved_rows().iter().take(24).collect(),
            3 => ui.get_recent_rows().iter().take(24).collect(),
            _ => ui.get_results().iter().take(24).collect(),
        };
        let mut keys: HashSet<_> = visible.iter().map(|row| row.id.to_string()).collect();
        if let Some(current) = self.state.borrow().current.clone() {
            keys.insert(current.id.to_string());
            self.request(current, ArtworkPriority::Current);
        }
        self.artwork.retain_visible(&keys);
        for row in visible {
            self.request(row, ArtworkPriority::Visible);
        }
    }
    fn start(&self, mut row: SearchRow) -> bool {
        self.flush();
        let Some(ui) = self.ui.upgrade() else {
            return false;
        };
        if let Some(image) = self
            .state
            .borrow()
            .images
            .iter()
            .find(|image| image.id == row.id)
        {
            row.artwork = image.artwork.clone();
        }
        if !command(
            &ui,
            &self.playback,
            PlaybackCommand::OpenRemote(crate::view::track(&row)),
        ) {
            return false;
        }
        {
            let mut state = self.state.borrow_mut();
            state.current = Some(row.clone());
            state.last_position_ms = 0;
            state.recent.begin();
        }
        ui.set_selected_id(row.id.clone());
        ui.set_track_title(row.title.clone());
        ui.set_track_artist(row.artist.clone());
        ui.set_track_artwork(row.artwork.clone());
        ui.set_details_open(true);
        ui.set_position_seconds(0.0);
        ui.set_elapsed_label("0:00".into());
        self.request(row, ArtworkPriority::Current);
        true
    }
    fn select(&self, row: SearchRow, queued: bool) {
        let Some(ui) = self.ui.upgrade() else {
            return;
        };
        if !self.start(row.clone()) {
            return;
        }
        let mut state = self.state.borrow_mut();
        if queued {
            if let Some(current) = state.cursor {
                remember(&mut state, current);
            }
        } else {
            let candidates = match ui.get_page() {
                2 => rows(ui.get_saved_rows()),
                3 => rows(ui.get_recent_rows()),
                0 => ui
                    .get_recent_rows()
                    .iter()
                    .chain(ui.get_trending_rows().iter())
                    .chain(ui.get_discovery_rows().iter())
                    .take(100)
                    .collect(),
                _ => rows(ui.get_results()),
            };
            let mut ids = HashSet::new();
            state.queue = candidates
                .into_iter()
                .filter(|item| ids.insert(item.id.clone()))
                .collect();
            if !state.queue.iter().any(|item| item.id == row.id) {
                state.queue = vec![row.clone()];
            }
            state.back.clear();
        }
        state.cursor = state.queue.iter().position(|item| item.id == row.id);
        drop(state);
        self.refresh();
    }
    fn advance(&self, ended: bool) {
        let Some(ui) = self.ui.upgrade() else {
            return;
        };
        let next = {
            let mut state = self.state.borrow_mut();
            let Some(current) = state.cursor else {
                return;
            };
            if state.queue.is_empty() {
                return;
            }
            if ended && ui.get_repeat() {
                current
            } else if ui.get_shuffle() && state.queue.len() > 1 {
                state.counter = state.counter.wrapping_add(1);
                (current
                    + 1
                    + state.random.hash_one(state.counter) as usize % (state.queue.len() - 1))
                    % state.queue.len()
            } else {
                current + 1
            }
        };
        let row = self.state.borrow().queue.get(next).cloned();
        if let Some(row) = row
            && self.start(row)
        {
            let mut state = self.state.borrow_mut();
            if let Some(current) = state.cursor
                && current != next
            {
                remember(&mut state, current);
            }
            state.cursor = Some(next);
        }
        self.refresh();
    }
    fn previous(&self) {
        let index = {
            let state = self.state.borrow();
            state
                .back
                .back()
                .copied()
                .or_else(|| state.cursor.and_then(|index| index.checked_sub(1)))
        };
        if let Some(index) = index {
            let row = self.state.borrow().queue.get(index).cloned();
            if let Some(row) = row
                && self.start(row)
            {
                let mut state = self.state.borrow_mut();
                if state.back.back() == Some(&index) {
                    state.back.pop_back();
                }
                state.cursor = Some(index);
            }
        }
        self.refresh();
    }
    fn toggle_saved(&self, row: SearchRow) {
        let Some(ui) = self.ui.upgrade() else {
            return;
        };
        if !ui.get_library_ready() {
            return;
        }
        let track = crate::view::track(&row);
        if !track.valid() {
            return;
        }
        {
            let mut state = self.state.borrow_mut();
            if let Some(index) = state
                .favorites
                .iter()
                .position(|saved| saved.id == track.id)
            {
                state.favorites.remove(index);
            } else if state.favorites.len() < 500 {
                state.favorites.push(track);
            } else {
                ui.set_error_message("Vous pouvez conserver jusqu'à 500 titres favoris.".into());
                return;
            }
            state.dirty = true;
        }
        self.flush();
        self.refresh();
    }
    fn observe(&self, position_ms: u64) {
        let Some(ui) = self.ui.upgrade() else {
            return;
        };
        let changed = {
            let mut state = self.state.borrow_mut();
            let Some(mut row) = state.current.clone() else {
                return;
            };
            if row.id != ui.get_selected_id() {
                return;
            }
            if ui.get_duration_seconds() > 0.0 {
                row.duration_ms = ui.get_duration_seconds() * 1000.0;
                state.current = Some(row.clone());
            }
            let track = crate::view::track(&row);
            if position_ms < state.last_position_ms {
                state.recent.begin();
            }
            state.last_position_ms = position_ms;
            let changed = state.recent.observe(&track, position_ms, now_ms());
            let position_changed = state.recent.position(&track.id, position_ms);
            state.dirty |= changed || position_changed;
            changed
        };
        if changed {
            self.flush();
            self.refresh();
        }
    }
    fn metadata(&self, row: SearchRow) {
        let track = crate::view::track(&row);
        if !track.valid() || track.thumbnail.is_none() {
            return;
        }
        let mut state = self.state.borrow_mut();
        if let Some(current) = state
            .current
            .as_mut()
            .filter(|current| current.id == row.id)
        {
            current.thumbnail = row.thumbnail.clone();
        }
        for queued in &mut state.queue {
            if queued.id == row.id {
                queued.thumbnail = row.thumbnail.clone();
            }
        }
        let mut changed = false;
        for saved in &mut state.favorites {
            if saved.id == track.id && saved.thumbnail != track.thumbnail {
                saved.thumbnail = track.thumbnail.clone();
                changed = true;
            }
        }
        for recent in &mut state.recent.entries {
            if recent.track.id == track.id && recent.track.thumbnail != track.thumbnail {
                recent.track.thumbnail = track.thumbnail.clone();
                changed = true;
            }
        }
        state.dirty |= changed;
        drop(state);
        self.request(row, ArtworkPriority::Current);
    }
    fn artwork(&self, row: SearchRow) {
        let Some(ui) = self.ui.upgrade() else {
            return;
        };
        let mut state = self.state.borrow_mut();
        state.images.retain(|image| image.id != row.id);
        let mut evicted_ids = Vec::new();
        state.images.push_back(row.clone());
        for queued in &mut state.queue {
            if queued.id == row.id {
                queued.artwork = row.artwork.clone();
            }
        }
        if let Some(current) = state.current.as_mut()
            && current.id == row.id
        {
            current.artwork = row.artwork.clone();
        }
        while state
            .images
            .iter()
            .map(|image| {
                let size = image.artwork.size();
                u64::from(size.width) * u64::from(size.height) * 4
            })
            .sum::<u64>()
            > 32 * 1024 * 1024
        {
            let Some(index) = state
                .images
                .iter()
                .position(|image| image.id != ui.get_selected_id())
            else {
                break;
            };
            let Some(evicted) = state.images.remove(index) else {
                break;
            };
            evicted_ids.push(evicted.id.clone());
            for queued in &mut state.queue {
                if queued.id == evicted.id {
                    queued.artwork = slint::Image::default();
                }
            }
        }
        drop(state);
        for id in evicted_ids {
            crate::view::image(&ui, &id, &slint::Image::default());
        }
        crate::view::image(&ui, &row.id, &row.artwork);
    }
}
fn remember(state: &mut State, index: usize) {
    if state.back.len() == 128 {
        state.back.pop_front();
    }
    state.back.push_back(index);
}

pub(crate) fn wire(
    ui: &AppWindow,
    playback: Arc<Playback>,
    artwork: Arc<Artwork>,
) -> std::io::Result<SessionHandle> {
    let library = Arc::new(Library::spawn(ui.as_weak())?);
    let c = Rc::new(Controller {
        ui: ui.as_weak(),
        state: RefCell::new(State {
            volume: 25.0,
            navigation: VecDeque::from([0]),
            ..Default::default()
        }),
        playback,
        library,
        artwork,
    });
    {
        let c = c.clone();
        ui.on_navigate(move |page| {
            if !(0..=3).contains(&page) {
                return;
            }
            let Some(ui) = c.ui.upgrade() else {
                return;
            };
            let mut state = c.state.borrow_mut();
            if state.navigation.get(state.navigation_cursor) != Some(&page) {
                let size = state.navigation_cursor + 1;
                state.navigation.truncate(size);
                state.navigation.push_back(page);
                if state.navigation.len() > 32 {
                    state.navigation.pop_front();
                }
                state.navigation_cursor = state.navigation.len() - 1;
            }
            ui.set_page(page);
            ui.set_can_go_back(state.navigation_cursor > 0);
            ui.set_can_go_forward(false);
            drop(state);
            c.request_visible();
        });
    }
    for forward in [false, true] {
        let c = c.clone();
        let navigate = move || {
            let Some(ui) = c.ui.upgrade() else {
                return;
            };
            let mut state = c.state.borrow_mut();
            if forward {
                if state.navigation_cursor + 1 >= state.navigation.len() {
                    return;
                }
                state.navigation_cursor += 1;
            } else {
                if state.navigation_cursor == 0 {
                    return;
                }
                state.navigation_cursor -= 1;
            }
            ui.set_page(state.navigation[state.navigation_cursor]);
            ui.set_can_go_back(state.navigation_cursor > 0);
            ui.set_can_go_forward(state.navigation_cursor + 1 < state.navigation.len());
            drop(state);
            c.request_visible();
        };
        if forward {
            ui.on_nav_forward(navigate);
        } else {
            ui.on_nav_back(navigate);
        }
    }
    {
        let c = c.clone();
        ui.on_library_loaded(move || {
            if let Some(snapshot) = c.library.take_loaded() {
                let mut state = c.state.borrow_mut();
                state.favorites = snapshot.favorites;
                state.library_loaded = true;
                let mut recent = std::mem::take(&mut state.recent.entries);
                recent.extend(snapshot.recent);
                recent.sort_by_key(|entry| std::cmp::Reverse(entry.played_at_unix_ms));
                let mut ids = HashSet::new();
                recent.retain(|entry| ids.insert(entry.track.id.clone()));
                recent.truncate(200);
                state.recent.entries = recent;
                drop(state);
                if let Some(ui) = c.ui.upgrade() {
                    ui.set_library_ready(true);
                }
                c.refresh();
            }
        });
    }
    {
        let c = c.clone();
        ui.on_play_track(move |row| c.select(row, false));
    }
    {
        let c = c.clone();
        ui.on_track_metadata(move |row| c.metadata(row));
    }
    {
        let c = c.clone();
        ui.on_play_queued(move |row| c.select(row, true));
    }
    {
        let c = c.clone();
        ui.on_next_track(move || c.advance(false));
    }
    {
        let c = c.clone();
        ui.on_ended(move || {
            c.flush();
            c.advance(true)
        });
    }
    {
        let c = c.clone();
        ui.on_previous_track(move || c.previous());
    }
    {
        let c = c.clone();
        ui.on_save_track(move |row| c.toggle_saved(row));
    }
    {
        let c = c.clone();
        ui.on_save_current(move || {
            let row = c.state.borrow().current.clone();
            if let Some(row) = row {
                c.toggle_saved(row);
            }
        });
    }
    {
        let c = c.clone();
        ui.on_artwork_ready(move |row| c.artwork(row));
    }
    {
        let c = c.clone();
        ui.on_request_artwork(move |row| c.request(row, ArtworkPriority::Visible));
    }
    {
        let c = c.clone();
        ui.on_refresh_images(move || c.request_visible());
    }
    {
        let c = c.clone();
        ui.on_audio_position(move |milliseconds| c.observe(milliseconds.max(0.0) as u64));
    }
    {
        let c = c.clone();
        ui.on_flush_state(move || c.flush());
    }
    {
        let c = c.clone();
        ui.on_queue_mode_changed(move || c.refresh());
    }
    {
        let c = c.clone();
        ui.on_clear_queue(move || {
            let mut state = c.state.borrow_mut();
            state.queue.clear();
            state.cursor = None;
            state.back.clear();
            drop(state);
            c.refresh();
        });
    }
    {
        let c = c.clone();
        ui.on_enqueue(move |row| {
            let Some(ui) = c.ui.upgrade() else {
                return;
            };
            let mut state = c.state.borrow_mut();
            if state.queue.len() >= 500 {
                ui.set_error_message("La file contient déjà 500 titres.".into());
                return;
            }
            if !state.queue.iter().any(|item| item.id == row.id) {
                state.queue.push(row);
            }
            drop(state);
            ui.set_queue_open(true);
            c.refresh();
        });
    }
    {
        let c = c.clone();
        ui.on_mute(move || {
            let Some(ui) = c.ui.upgrade() else {
                return;
            };
            let mut state = c.state.borrow_mut();
            let value = if ui.get_volume() > 0.0 {
                state.volume = ui.get_volume();
                0.0
            } else {
                state.volume.max(1.0)
            };
            drop(state);
            ui.set_volume(value);
            command(&ui, &c.playback, PlaybackCommand::SetVolume(value / 100.0));
        });
    }
    Ok(SessionHandle(c))
}
