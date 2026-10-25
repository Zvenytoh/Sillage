use crate::app::AppWindow;
use sillage::history::LibrarySnapshot;
use sillage::model::Track;
use std::fs::{self, File, OpenOptions};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::JoinHandle;

#[derive(Default)]
struct State {
    pending: Option<LibrarySnapshot>,
    loaded: Option<LibrarySnapshot>,
    stopped: bool,
}
pub(crate) struct Library {
    shared: Arc<(Mutex<State>, Condvar)>,
    ready: Arc<AtomicBool>,
    worker: Option<JoinHandle<()>>,
}

impl Library {
    pub fn spawn(ui: slint::Weak<AppWindow>) -> std::io::Result<Self> {
        let directory = crate::cache::directory()
            .ok_or_else(|| std::io::Error::other("Le dossier de données est indisponible."))?;
        let shared = Arc::new((Mutex::new(State::default()), Condvar::new()));
        let worker_shared = shared.clone();
        let ready = Arc::new(AtomicBool::new(false));
        let worker_ready = ready.clone();
        let worker=std::thread::Builder::new().name("sillage-library".into()).spawn(move|| {
            let Ok(_lock)=library_lock(&directory) else {
                let _=ui.upgrade_in_event_loop(|ui|ui.set_error_message("La bibliothèque est indisponible ou utilisée dans une autre fenêtre. Vos données sont conservées.".into())); return;
            };
            let path=directory.join("library.json");
            let snapshot=match load(&path) { Ok(snapshot)=>snapshot, Err(())=>{
                let _=ui.upgrade_in_event_loop(|ui|ui.set_error_message("La bibliothèque ne peut pas être lue. Le fichier existant est conservé et la sauvegarde désactivée.".into())); return;
            }};
            if let Ok(mut state)=worker_shared.0.lock() { state.loaded=Some(snapshot); } else { return; }
            worker_ready.store(true,Ordering::Release);
            if ui.upgrade_in_event_loop(|ui|ui.invoke_library_loaded()).is_err() { return; }
            loop {
                let Ok(state)=worker_shared.0.lock() else { return; };
                let Ok(mut state)=worker_shared.1.wait_while(state,|state|state.pending.is_none()&&!state.stopped) else { return; };
                let Some(snapshot)=state.pending.take() else { return; }; drop(state);
                if save(&path,snapshot).is_err() {
                    let _=ui.upgrade_in_event_loop(|ui|ui.set_error_message("La sauvegarde a échoué. Le fichier précédent est conservé ; vos changements restent dans cette session.".into()));
                }
            }
        })?;
        Ok(Self {
            shared,
            ready,
            worker: Some(worker),
        })
    }
    pub fn take_loaded(&self) -> Option<LibrarySnapshot> {
        self.shared.0.lock().ok()?.loaded.take()
    }
    pub fn save(&self, snapshot: &LibrarySnapshot) -> Result<(), &'static str> {
        if !self.ready.load(Ordering::Acquire) {
            return Err("La bibliothèque n'est pas encore prête.");
        }
        let mut state = self
            .shared
            .0
            .lock()
            .map_err(|_| "La sauvegarde est indisponible.")?;
        state.pending = Some(snapshot.clone());
        self.shared.1.notify_one();
        Ok(())
    }
}
impl Drop for Library {
    fn drop(&mut self) {
        if let Ok(mut state) = self.shared.0.lock() {
            state.stopped = true;
        }
        self.shared.1.notify_one();
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
        options.share_mode(0);
    }
    options.open(directory.join("library.lock"))
}
fn validate(snapshot: &LibrarySnapshot) -> bool {
    let unique = |tracks: Vec<&Track>| {
        let mut ids = std::collections::HashSet::new();
        tracks
            .iter()
            .all(|track| track.valid() && ids.insert(&track.id))
    };
    snapshot.favorites.len() <= 500
        && snapshot.recent.len() <= 200
        && unique(snapshot.favorites.iter().collect())
        && unique(snapshot.recent.iter().map(|recent| &recent.track).collect())
}
fn load(path: &Path) -> Result<LibrarySnapshot, ()> {
    let bytes = match crate::cache::read(path, 2 * 1024 * 1024) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(LibrarySnapshot::default());
        }
        Err(_) => return Err(()),
    };
    let value: serde_json::Value = serde_json::from_slice(&bytes).map_err(|_| ())?;
    let snapshot = match value["version"].as_u64() {
        Some(1) => {
            let items = value["tracks"]
                .as_array()
                .filter(|items| items.len() <= 500)
                .ok_or(())?;
            let favorites = items
                .iter()
                .map(|item| {
                    Ok(Track {
                        id: item["id"].as_str().ok_or(())?.into(),
                        title: item["title"].as_str().ok_or(())?.into(),
                        artists: vec![item["artist"].as_str().ok_or(())?.into()],
                        duration_ms: None,
                        thumbnail: None,
                    })
                })
                .collect::<Result<Vec<_>, ()>>()?;
            LibrarySnapshot {
                favorites,
                recent: Vec::new(),
            }
        }
        Some(2) => serde_json::from_value(value).map_err(|_| ())?,
        _ => return Err(()),
    };
    if validate(&snapshot) {
        Ok(snapshot)
    } else {
        Err(())
    }
}
fn save(path: &Path, snapshot: LibrarySnapshot) -> Result<(), ()> {
    if !validate(&snapshot) {
        return Err(());
    }
    let bytes = serde_json::to_vec(
        &serde_json::json!({"version":2,"favorites":snapshot.favorites,"recent":snapshot.recent}),
    )
    .map_err(|_| ())?;
    if bytes.len() > 2 * 1024 * 1024 {
        return Err(());
    }
    // Keep the established recovery filename and never overwrite a pre-existing pending file.
    let pending = path.with_extension("json.pending");
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&pending)
        .map_err(|_| ())?;
    use std::io::Write;
    let result = file.write_all(&bytes).and_then(|()| file.sync_all());
    drop(file);
    if result.is_err() || fs::rename(&pending, path).is_err() {
        let _ = fs::remove_file(pending);
        return Err(());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
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
    fn v1_migration_preserves_favorites() {
        let fixture = Fixture::new();
        let path = fixture.0.join("library.json");
        let bytes =
            br#"{"version":1,"tracks":[{"id":"abcdefghijk","title":"Titre","artist":"Artiste"}]}"#;
        fs::write(&path, bytes).expect("fixture");
        let snapshot = load(&path).expect("v1 supported");
        assert_eq!(snapshot.favorites[0].title, "Titre");
        assert_eq!(snapshot.favorites[0].artists, ["Artiste"]);
        assert!(snapshot.recent.is_empty());
        assert_eq!(fs::read(&path).expect("read"), bytes);
        save(&path, snapshot.clone()).expect("v2 write");
        assert_eq!(load(&path).expect("v2 read").favorites, snapshot.favorites);
    }
    #[test]
    fn pending_recovery_file_prevents_overwriting_previous_library() {
        let fixture = Fixture::new();
        let path = fixture.0.join("library.json");
        fs::write(&path, b"previous-library").expect("previous");
        fs::write(path.with_extension("json.pending"), b"recovery").expect("pending");
        assert!(save(&path, LibrarySnapshot::default()).is_err());
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
