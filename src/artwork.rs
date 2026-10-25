use crate::app::AppWindow;
use sillage::model::Track;
use sillage::service::{Cancellation, Client, ServiceError};
use std::collections::{HashSet, VecDeque};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::JoinHandle;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArtworkPriority {
    Current,
    Visible,
}
struct Job {
    track: Track,
    priority: ArtworkPriority,
}
#[derive(Default)]
struct State {
    pending: VecDeque<Job>,
    active: Option<String>,
    stopped: bool,
}
pub(crate) struct Artwork {
    shared: Arc<(Mutex<State>, Condvar)>,
    cancel: Cancellation,
    worker: Option<JoinHandle<()>>,
}

impl Artwork {
    pub fn new(ui: slint::Weak<AppWindow>) -> std::io::Result<Self> {
        let shared = Arc::new((Mutex::new(State::default()), Condvar::new()));
        let worker_shared = shared.clone();
        let cancel = Cancellation::default();
        let stopping = cancel.clone();
        let worker = std::thread::Builder::new()
            .name("sillage-artwork".into())
            .spawn(move || {
                let client = Client::default();
                loop {
                    let Ok(state) = worker_shared.0.lock() else {
                        return;
                    };
                    let Ok(mut state) = worker_shared
                        .1
                        .wait_while(state, |state| state.pending.is_empty() && !state.stopped)
                    else {
                        return;
                    };
                    if state.stopped {
                        return;
                    }
                    let Some(job) = state.pending.pop_front() else {
                        continue;
                    };
                    state.active = Some(job.track.id.clone());
                    drop(state);
                    let path = crate::cache::directory().map(|root| {
                        root.join("artwork-v2")
                            .join(format!("{}.img", job.track.id))
                    });
                    let cached = path
                        .as_ref()
                        .and_then(|path| crate::cache::read(path, 512 * 1024).ok())
                        .and_then(|bytes| decode(&bytes, job.priority == ArtworkPriority::Current));
                    let pixels = if cached.is_some() {
                        cached
                    } else {
                        let fallback =
                            format!("https://i.ytimg.com/vi/{}/hqdefault.jpg", job.track.id);
                        let first = job
                            .track
                            .thumbnail
                            .as_deref()
                            .filter(|url| sillage::service::valid_artwork_url(url))
                            .unwrap_or(&fallback);
                        let mut result = client.artwork(first, &stopping);
                        if result.is_err()
                            && first != fallback
                            && !matches!(
                                result,
                                Err(ServiceError::RateLimited | ServiceError::Cancelled)
                            )
                        {
                            result = client.artwork(&fallback, &stopping);
                        }
                        match result {
                            Ok(bytes) => {
                                let pixels =
                                    decode(&bytes, job.priority == ArtworkPriority::Current);
                                if pixels.is_some()
                                    && let Some(path) = &path
                                {
                                    let _ = crate::cache::write(path, &bytes);
                                    prune_disk(path.parent());
                                }
                                pixels
                            }
                            Err(ServiceError::RateLimited) => {
                                if let Ok(mut state) = worker_shared.0.lock() {
                                    state.pending.clear();
                                }
                                None
                            }
                            Err(_) => None,
                        }
                    };
                    if let Ok(mut state) = worker_shared.0.lock() {
                        state.active = None;
                    }
                    if stopping.check().is_err() {
                        return;
                    }
                    if let Some(pixels) = pixels
                        && ui
                            .upgrade_in_event_loop(move |ui| {
                                let mut row = crate::view::row(&job.track);
                                row.artwork = slint::Image::from_rgba8(pixels);
                                ui.invoke_artwork_ready(row);
                            })
                            .is_err()
                    {
                        return;
                    }
                }
            })?;
        Ok(Self {
            shared,
            cancel,
            worker: Some(worker),
        })
    }
    pub fn request(&self, track: &Track, priority: ArtworkPriority) {
        if !track.valid() {
            return;
        }
        if let Ok(mut state) = self.shared.0.lock() {
            if state.active.as_deref() == Some(&track.id) {
                return;
            }
            if let Some(index) = state
                .pending
                .iter()
                .position(|job| job.track.id == track.id)
            {
                if state.pending[index].priority <= priority {
                    return;
                }
                state.pending.remove(index);
            }
            if state.pending.len() >= 64 {
                state.pending.pop_back();
            }
            let index = state
                .pending
                .iter()
                .position(|job| job.priority > priority)
                .unwrap_or(state.pending.len());
            state.pending.insert(
                index,
                Job {
                    track: track.clone(),
                    priority,
                },
            );
            self.shared.1.notify_one();
        }
    }
    pub fn retain_visible(&self, keys: &HashSet<String>) {
        if let Ok(mut state) = self.shared.0.lock() {
            state.pending.retain(|job| {
                job.priority == ArtworkPriority::Current || keys.contains(&job.track.id)
            });
        }
    }
}
impl Drop for Artwork {
    fn drop(&mut self) {
        self.cancel.cancel();
        if let Ok(mut state) = self.shared.0.lock() {
            state.stopped = true;
            state.pending.clear();
        }
        self.shared.1.notify_one();
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}
fn decode(bytes: &[u8], current: bool) -> Option<slint::SharedPixelBuffer<slint::Rgba8Pixel>> {
    let mut reader = image::ImageReader::new(std::io::Cursor::new(bytes))
        .with_guessed_format()
        .ok()?;
    let mut limits = image::Limits::default();
    limits.max_image_width = Some(2048);
    limits.max_image_height = Some(2048);
    limits.max_alloc = Some(16 * 1024 * 1024);
    reader.limits(limits);
    let side = if current { 512 } else { 384 };
    let image = reader.decode().ok()?.thumbnail(side, side).to_rgba8();
    Some(slint::SharedPixelBuffer::clone_from_slice(
        image.as_raw(),
        image.width(),
        image.height(),
    ))
}
fn prune_disk(directory: Option<&std::path::Path>) {
    let Some(directory) = directory else {
        return;
    };
    let Ok(items) = std::fs::read_dir(directory) else {
        return;
    };
    let mut files: Vec<_> = items
        .take(4096)
        .filter_map(Result::ok)
        .filter_map(|item| {
            let path = item.path();
            if path.extension()?.to_str()? != "img"
                || !sillage::service::parse::valid_id(path.file_stem()?.to_str()?)
            {
                return None;
            }
            let meta = item.metadata().ok()?;
            if !meta.is_file() {
                return None;
            }
            Some((meta.modified().ok()?, meta.len(), path))
        })
        .collect();
    let mut bytes: u64 = files.iter().map(|item| item.1).sum();
    let mut count = files.len();
    files.sort_by_key(|item| item.0);
    for (_, size, path) in files {
        if bytes <= 128 * 1024 * 1024 && count <= 1024 {
            break;
        }
        if std::fs::remove_file(path).is_ok() {
            bytes = bytes.saturating_sub(size);
            count = count.saturating_sub(1);
        }
    }
}
