use crate::app::{AppWindow, SearchRow};
use sillage::model::Track;
use sillage::service::parse::valid_id;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};
use std::thread::JoinHandle;

pub(crate) struct Library {
    sender: Option<mpsc::SyncSender<Vec<Track>>>,
    ready: Arc<AtomicBool>,
    worker: Option<JoinHandle<()>>,
}

impl Library {
    pub fn spawn(ui: slint::Weak<AppWindow>) -> std::io::Result<Self> {
        let directory = std::env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .ok_or_else(|| std::io::Error::other("Local application data is unavailable."))?
            .join("Sillage");
        let (sender, receiver) = mpsc::sync_channel::<Vec<Track>>(8);
        let ready = Arc::new(AtomicBool::new(false));
        let worker_ready = ready.clone();
        let worker = std::thread::Builder::new().name("sillage-library".into()).spawn(move || {
            let _lock = match library_lock(&directory) {
                Ok(lock) => lock,
                Err(_) => {
                    let _ = ui.upgrade_in_event_loop(|ui| ui.set_error_message("Local library storage is unavailable or already in use by another Sillage window. Saving is disabled in this window.".into()));
                    return;
                }
            };
            let path = directory.join("library.json");
            match load(&path) {
                Ok(tracks) => {
                    worker_ready.store(true, Ordering::Release);
                    if ui.upgrade_in_event_loop(move |ui| {
                        let rows: Vec<_> = tracks.into_iter().map(|track| SearchRow {
                            id: track.id.into(), title: track.title.into(), artist: track.artists.join(" · ").into(),
                            artwork: slint::Image::default(),
                            duration: track.duration_ms.map(|ms| format!("{}:{:02}", ms / 60000, ms / 1000 % 60)).unwrap_or_default().into(),
                        }).collect();
                        ui.set_saved_rows(slint::ModelRc::new(slint::VecModel::from(rows)));
                        ui.set_library_ready(true);
                    }).is_err() { return; }
                }
                Err(()) => {
                    let _ = ui.upgrade_in_event_loop(|ui| ui.set_error_message("Your saved library could not be read. The existing file has been preserved; saving is disabled.".into()));
                    return;
                }
            }
            while let Ok(tracks) = receiver.recv() {
                if save(&path, tracks).is_err() {
                    let _ = ui.upgrade_in_event_loop(|ui| ui.set_error_message("Your library could not be saved. Your previous file is preserved; the latest changes remain in this session.".into()));
                }
            }
        })?;
        Ok(Self {
            sender: Some(sender),
            ready,
            worker: Some(worker),
        })
    }

    pub fn save(&self, rows: &[SearchRow]) -> Result<(), &'static str> {
        if !self.ready.load(Ordering::Acquire) {
            return Err("Your library is not ready for saving.");
        }
        let tracks = rows
            .iter()
            .map(|row| Track {
                id: row.id.to_string(),
                title: row.title.to_string(),
                artists: vec![row.artist.to_string()],
                duration_ms: None,
                thumbnail: None,
            })
            .collect();
        self.sender
            .as_ref()
            .ok_or("Library storage is unavailable.")?
            .try_send(tracks)
            .map_err(|_| "Library storage is busy. Try saving again.")
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        self.sender.take();
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

fn library_lock(directory: &Path) -> std::io::Result<File> {
    fs::create_dir_all(directory)?;
    let mut options = OpenOptions::new();
    options.read(true).write(true).create(true).truncate(false);
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        // Windows releases the exclusive sharing handle even after an abnormal exit.
        options.share_mode(0);
    }
    options.open(directory.join("library.lock"))
}

fn load(path: &Path) -> Result<Vec<Track>, ()> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(_) => return Err(()),
    };
    let mut bytes = Vec::new();
    file.take(2 * 1024 * 1024 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| ())?;
    if bytes.len() > 2 * 1024 * 1024 {
        return Err(());
    }
    let value: serde_json::Value = serde_json::from_slice(&bytes).map_err(|_| ())?;
    if value["version"].as_u64() != Some(1) {
        return Err(());
    }
    let tracks = value["tracks"]
        .as_array()
        .filter(|tracks| tracks.len() <= 500)
        .ok_or(())?;
    let mut seen = std::collections::HashSet::new();
    tracks
        .iter()
        .map(|track| {
            let id = track["id"]
                .as_str()
                .filter(|id| valid_id(id) && seen.insert(id.to_string()))
                .ok_or(())?;
            let title = track["title"]
                .as_str()
                .filter(|title| !title.is_empty() && title.chars().count() <= 512)
                .ok_or(())?;
            let artist = track["artist"]
                .as_str()
                .filter(|artist| artist.chars().count() <= 4096)
                .ok_or(())?;
            Ok(Track {
                id: id.to_string(),
                title: title.to_string(),
                artists: vec![artist.to_string()],
                duration_ms: None,
                thumbnail: None,
            })
        })
        .collect()
}

fn save(path: &Path, tracks: Vec<Track>) -> Result<(), ()> {
    if tracks.len() > 500 {
        return Err(());
    }
    let tracks: Vec<_> = tracks.into_iter().map(|track| serde_json::json!({ "id": track.id, "title": track.title, "artist": track.artists.join(" · ") })).collect();
    let bytes = serde_json::to_vec(&serde_json::json!({ "version": 1, "tracks": tracks }))
        .map_err(|_| ())?;
    if bytes.len() > 2 * 1024 * 1024 {
        return Err(());
    }
    fs::create_dir_all(path.parent().ok_or(())?).map_err(|_| ())?;
    let pending = path.with_extension("json.pending");
    // CreateNew refuses an existing recovery file; only our own new temporary file may be removed.
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&pending)
        .map_err(|_| ())?;
    let result = file.write_all(&bytes).and_then(|()| file.sync_all());
    drop(file);
    if result.is_err() || fs::rename(&pending, path).is_err() {
        let _ = fs::remove_file(&pending);
        return Err(());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            let nonce = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("clock")
                .as_nanos();
            let directory = std::env::temp_dir().join(format!(
                "sillage-library-test-{}-{nonce}",
                std::process::id()
            ));
            fs::create_dir(&directory).expect("isolated directory");
            Self(directory)
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            for name in ["library.json", "library.json.pending"] {
                let _ = fs::remove_file(self.0.join(name));
            }
            let _ = fs::remove_dir(&self.0);
        }
    }
    #[test]
    fn pending_recovery_file_prevents_overwriting_previous_library() {
        let fixture = Fixture::new();
        let path = fixture.0.join("library.json");
        fs::write(&path, b"previous-library").expect("previous");
        fs::write(path.with_extension("json.pending"), b"recovery").expect("pending");
        assert!(save(&path, Vec::new()).is_err());
        assert_eq!(
            fs::read(&path).expect("previous preserved"),
            b"previous-library"
        );
        assert_eq!(
            fs::read(path.with_extension("json.pending")).expect("recovery preserved"),
            b"recovery"
        );
    }
    #[test]
    fn unknown_library_version_is_rejected_without_modifying_the_file() {
        let fixture = Fixture::new();
        let path = fixture.0.join("library.json");
        let bytes = br#"{"version":99,"tracks":[]}"#;
        fs::write(&path, bytes).expect("fixture");
        assert!(load(&path).is_err());
        assert_eq!(fs::read(&path).expect("preserved"), bytes);
    }
}
