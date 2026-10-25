use crate::app::{AppWindow, SearchRow};
use sillage::service::{Cancellation, Client, ServiceError};
use slint::Model;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::JoinHandle;

type Batch = (Vec<(String, String)>, Cancellation);
#[derive(Default)]
struct State {
    pending: Option<Batch>,
    active: Cancellation,
    stopped: bool,
}
pub(crate) struct Artwork {
    shared: Arc<(Mutex<State>, Condvar)>,
    worker: Option<JoinHandle<()>>,
}

impl Artwork {
    pub fn new(ui: slint::Weak<AppWindow>) -> std::io::Result<Self> {
        let shared = Arc::new((Mutex::new(State::default()), Condvar::new()));
        let worker_shared = shared.clone();
        let worker = std::thread::Builder::new()
            .name("sillage-artwork".into())
            .spawn(move || {
                let client = Client::default();
                loop {
                    let (lock, changed) = &*worker_shared;
                    let Ok(state) = lock.lock() else {
                        return;
                    };
                    let Ok(mut state) = changed
                        .wait_while(state, |state| state.pending.is_none() && !state.stopped)
                    else {
                        return;
                    };
                    if state.stopped {
                        return;
                    }
                    let Some((thumbnails, cancel)) = state.pending.take() else {
                        continue;
                    };
                    state.active = cancel.clone();
                    drop(state);
                    let mut failures = 0;
                    for (id, url) in thumbnails {
                        if cancel.check().is_err() {
                            break;
                        }
                        let bytes = match client.artwork(&url, &cancel) {
                            Ok(bytes) => {
                                failures = 0;
                                bytes
                            }
                            Err(ServiceError::Cancelled | ServiceError::RateLimited) => break,
                            Err(_) => {
                                failures += 1;
                                if failures >= 4 {
                                    break;
                                }
                                continue;
                            }
                        };
                        let Some(pixels) = decode(bytes) else {
                            continue;
                        };
                        let image_cancel = cancel.clone();
                        if ui
                            .upgrade_in_event_loop(move |ui| {
                                if image_cancel.check().is_err() {
                                    return;
                                }
                                let rows = ui.get_results();
                                let Some(model) =
                                    rows.as_any().downcast_ref::<slint::VecModel<SearchRow>>()
                                else {
                                    return;
                                };
                                for index in 0..model.row_count() {
                                    if let Some(mut row) = model.row_data(index)
                                        && row.id.as_str() == id
                                    {
                                        row.artwork = slint::Image::from_rgba8(pixels);
                                        if ui.get_selected_id() == row.id {
                                            ui.set_track_artwork(row.artwork.clone());
                                        }
                                        model.set_row_data(index, row.clone());
                                        ui.invoke_artwork_ready(row);
                                        break;
                                    }
                                }
                            })
                            .is_err()
                        {
                            return;
                        }
                    }
                }
            })?;
        Ok(Self {
            shared,
            worker: Some(worker),
        })
    }
    pub fn submit(&self, thumbnails: Vec<(String, String)>, cancel: Cancellation) {
        let (lock, changed) = &*self.shared;
        if let Ok(mut state) = lock.lock() {
            state.active.cancel();
            state.pending = Some((thumbnails, cancel));
            changed.notify_one();
        }
    }
}
impl Drop for Artwork {
    fn drop(&mut self) {
        let (lock, changed) = &*self.shared;
        if let Ok(mut state) = lock.lock() {
            state.stopped = true;
            state.active.cancel();
            if let Some((_, cancel)) = &state.pending {
                cancel.cancel();
            }
        }
        changed.notify_one();
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}
fn decode(bytes: Vec<u8>) -> Option<slint::SharedPixelBuffer<slint::Rgba8Pixel>> {
    let mut reader = image::ImageReader::new(std::io::Cursor::new(bytes))
        .with_guessed_format()
        .ok()?;
    let mut limits = image::Limits::default();
    limits.max_image_width = Some(2048);
    limits.max_image_height = Some(2048);
    limits.max_alloc = Some(16 * 1024 * 1024);
    reader.limits(limits);
    let image = reader.decode().ok()?.thumbnail(320, 320).to_rgba8();
    Some(slint::SharedPixelBuffer::clone_from_slice(
        image.as_raw(),
        image.width(),
        image.height(),
    ))
}
