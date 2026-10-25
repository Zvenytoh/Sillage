use crate::app::{AppWindow, SearchRow};
use sillage::service::{Cancellation, Client};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::JoinHandle;

struct State {
    pending: Option<(String, Cancellation)>,
    current: Cancellation,
    stopped: bool,
}
struct Shared {
    state: Mutex<State>,
    changed: Condvar,
}

pub(crate) struct Search {
    shared: Arc<Shared>,
    worker: Option<JoinHandle<()>>,
}

impl Search {
    pub fn new(ui: slint::Weak<AppWindow>) -> std::io::Result<Self> {
        let shared = Arc::new(Shared {
            state: Mutex::new(State {
                pending: None,
                current: Cancellation::default(),
                stopped: false,
            }),
            changed: Condvar::new(),
        });
        let worker_shared = shared.clone();
        let worker = std::thread::Builder::new()
            .name("sillage-search".into())
            .spawn(move || {
                let client = Client::default();
                loop {
                    let Ok(state) = worker_shared.state.lock() else {
                        return;
                    };
                    let Ok(mut state) = worker_shared
                        .changed
                        .wait_while(state, |state| state.pending.is_none() && !state.stopped)
                    else {
                        return;
                    };
                    if state.stopped {
                        return;
                    }
                    let Some((query, cancel)) = state.pending.take() else {
                        continue;
                    };
                    drop(state);
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
                                    let rows: Vec<_> = tracks
                                        .into_iter()
                                        .map(|track| SearchRow {
                                            id: track.id.into(),
                                            title: track.title.into(),
                                            artist: track.artists.join(" · ").into(),
                                        })
                                        .collect();
                                    ui.set_results(slint::ModelRc::new(slint::VecModel::from(
                                        rows,
                                    )));
                                    ui.set_search_error("".into());
                                }
                                Err(error) => ui.set_search_error(error.to_string().into()),
                            }
                        })
                        .is_err()
                    {
                        return;
                    }
                }
            })?;
        Ok(Self {
            shared,
            worker: Some(worker),
        })
    }

    pub fn cancel(&self) {
        if let Ok(mut state) = self.shared.state.lock() {
            state.current.cancel();
            state.pending = None;
        }
    }

    pub fn submit(&self, query: String) -> Result<(), &'static str> {
        let query = query.trim();
        if query.is_empty() || query.chars().count() > 120 {
            return Err("Enter between 1 and 120 characters.");
        }
        let mut state = self
            .shared
            .state
            .lock()
            .map_err(|_| "Search is unavailable. Restart Sillage.")?;
        state.current.cancel();
        state.current = Cancellation::default();
        state.pending = Some((query.to_owned(), state.current.clone()));
        self.shared.changed.notify_one();
        Ok(())
    }
}

impl Drop for Search {
    fn drop(&mut self) {
        if let Ok(mut state) = self.shared.state.lock() {
            state.stopped = true;
            state.current.cancel();
            state.pending = None;
        }
        self.shared.changed.notify_one();
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}
