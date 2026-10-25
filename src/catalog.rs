use crate::app::AppWindow;
use sillage::model::HomePage;
use sillage::service::{Cancellation, Client};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::JoinHandle;

enum Request {
    Search(String),
    Home { force: bool },
}
struct State {
    pending: Option<(Request, Cancellation)>,
    current: Cancellation,
    stopped: bool,
}
pub(crate) struct Catalog {
    shared: Arc<(Mutex<State>, Condvar)>,
    worker: Option<JoinHandle<()>>,
}

impl Catalog {
    pub fn new(ui: slint::Weak<AppWindow>) -> std::io::Result<Self> {
        let shared = Arc::new((
            Mutex::new(State {
                pending: None,
                current: Cancellation::default(),
                stopped: false,
            }),
            Condvar::new(),
        ));
        let worker_shared = shared.clone();
        let worker = std::thread::Builder::new()
            .name("sillage-catalog".into())
            .spawn(move || {
                let client = Client::default();
                loop {
                    let Ok(state) = worker_shared.0.lock() else {
                        return;
                    };
                    let Ok(mut state) = worker_shared
                        .1
                        .wait_while(state, |state| state.pending.is_none() && !state.stopped)
                    else {
                        return;
                    };
                    if state.stopped {
                        return;
                    }
                    let Some((request, cancel)) = state.pending.take() else {
                        continue;
                    };
                    drop(state);
                    match request {
                        Request::Search(query) => {
                            let result = client.search(&query, &cancel);
                            if cancel.check().is_err() {
                                continue;
                            }
                            if ui
                                .upgrade_in_event_loop(move |ui| {
                                    if cancel.check().is_err() {
                                        return;
                                    }
                                    ui.set_searching(false);
                                    ui.set_searched(true);
                                    match result {
                                        Ok(tracks) => {
                                            ui.set_results(crate::view::model(&tracks));
                                            ui.set_search_error("".into());
                                            ui.invoke_refresh_images();
                                        }
                                        Err(error) => ui.set_search_error(error.to_string().into()),
                                    }
                                })
                                .is_err()
                            {
                                return;
                            }
                        }
                        Request::Home { force } => {
                            let path = crate::cache::directory()
                                .map(|root| root.join("home-fr-square-v2.json"));
                            let cached = path
                                .as_ref()
                                .and_then(|path| crate::cache::read(path, 2 * 1024 * 1024).ok())
                                .and_then(|bytes| serde_json::from_slice::<HomePage>(&bytes).ok())
                                .filter(valid_home);
                            let fresh = cached.as_ref().is_some_and(|page| {
                                sillage::model::now_ms().saturating_sub(page.fetched_at_unix_ms)
                                    < 6 * 3600 * 1000
                            });
                            if let Some(page) = cached.clone() {
                                let token = cancel.clone();
                                let _ = ui.upgrade_in_event_loop(move |ui| {
                                    if token.check().is_ok() {
                                        display_home(&ui, &page, true);
                                    }
                                });
                            }
                            if fresh && !force {
                                let token = cancel.clone();
                                let _ = ui.upgrade_in_event_loop(move |ui| {
                                    if token.check().is_ok() {
                                        ui.set_home_loading(false);
                                    }
                                });
                                continue;
                            }
                            let result = client.home("FR", &cancel);
                            if cancel.check().is_err() {
                                continue;
                            }
                            if let Ok(page) = &result
                                && valid_home(page)
                                && let Some(path) = path
                                && let Ok(bytes) = serde_json::to_vec(page)
                            {
                                let _ = crate::cache::write(&path, &bytes);
                            }
                            if ui
                                .upgrade_in_event_loop(move |ui| {
                                    if cancel.check().is_err() {
                                        return;
                                    }
                                    ui.set_home_loading(false);
                                    match result {
                                        Ok(page) => display_home(&ui, &page, false),
                                        Err(error) => {
                                            ui.set_home_error(
                                                if cached.is_some() {
                                                    format!("Mode hors ligne · {}", error)
                                                } else {
                                                    error.to_string()
                                                }
                                                .into(),
                                            );
                                        }
                                    }
                                })
                                .is_err()
                            {
                                return;
                            }
                        }
                    }
                }
            })?;
        Ok(Self {
            shared,
            worker: Some(worker),
        })
    }
    fn submit(&self, request: Request) -> Result<(), &'static str> {
        let mut state = self
            .shared
            .0
            .lock()
            .map_err(|_| "Le catalogue est indisponible.")?;
        state.current.cancel();
        state.current = Cancellation::default();
        state.pending = Some((request, state.current.clone()));
        self.shared.1.notify_one();
        Ok(())
    }
    pub fn search(&self, query: String) -> Result<(), &'static str> {
        let query = query.trim();
        if query.is_empty() || query.chars().count() > 120 {
            return Err("Saisissez entre 1 et 120 caractères.");
        }
        self.submit(Request::Search(query.into()))
    }
    pub fn home(&self, force: bool) -> Result<(), &'static str> {
        self.submit(Request::Home { force })
    }
    pub fn cancel(&self) {
        if let Ok(mut state) = self.shared.0.lock() {
            state.current.cancel();
            state.pending = None;
        }
    }
}
impl Drop for Catalog {
    fn drop(&mut self) {
        if let Ok(mut state) = self.shared.0.lock() {
            state.stopped = true;
            state.current.cancel();
            state.pending = None;
        }
        self.shared.1.notify_one();
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}
fn valid_home(page: &HomePage) -> bool {
    page.country == "FR"
        && page.fetched_at_unix_ms > 0
        && page.fetched_at_unix_ms <= sillage::model::now_ms().saturating_add(300_000)
        && page.trending.len() <= 20
        && page.discovery.len() <= 24
        && page
            .trending
            .iter()
            .chain(&page.discovery)
            .all(|track| track.valid())
        && page.source.len() <= 256
        && page
            .warning
            .as_ref()
            .is_none_or(|value| value.len() <= 1024)
}
fn display_home(ui: &AppWindow, page: &HomePage, cached: bool) {
    ui.set_trending_rows(crate::view::model(&page.trending));
    ui.set_discovery_rows(crate::view::model(&page.discovery));
    let source = if cached {
        format!("{} · sélection enregistrée", page.source)
    } else {
        page.source.clone()
    };
    ui.set_home_source(source.into());
    ui.set_home_error(page.warning.clone().unwrap_or_default().into());
    ui.invoke_refresh_images();
}
