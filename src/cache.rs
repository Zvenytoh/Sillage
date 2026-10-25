use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub fn directory() -> Option<PathBuf> {
    std::env::var_os("LOCALAPPDATA").map(|path| PathBuf::from(path).join("Sillage"))
}
pub fn read(path: &Path, limit: usize) -> std::io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    std::fs::File::open(path)?
        .take(limit as u64 + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() > limit {
        return Err(std::io::Error::other("cache_size"));
    }
    Ok(bytes)
}
pub fn write(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    std::fs::create_dir_all(
        path.parent()
            .ok_or_else(|| std::io::Error::other("cache_path"))?,
    )?;
    let pending = path.with_extension("pending");
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&pending)?;
    let result = file.write_all(bytes).and_then(|()| file.sync_all());
    drop(file);
    if let Err(error) = result.and_then(|()| std::fs::rename(&pending, path)) {
        let _ = std::fs::remove_file(pending);
        return Err(error);
    }
    Ok(())
}
