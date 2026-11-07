use log::{info, warn};
use std::{io::ErrorKind, path::Path};

/// Delete a post's media: all variants for images (`.webp`), the single file for videos.
/// Missing files are ignored; other IO errors are logged so the DB delete can still proceed.
pub fn remove_media_files(upload_dir: &Path, media_name: &str) {
    if media_name.is_empty() || media_name.contains(['/', '\\']) || media_name.contains("..") {
        warn!("Refusing to delete suspicious media name: {media_name:?}");
        return;
    }
    let names = match media_name.strip_suffix(".webp") {
        Some(base) => vec![
            format!("{base}.webp"),
            format!("{base}.jxl"),
            format!("{base}-800.webp"),
            format!("{base}-800.jxl"),
        ],
        None => vec![media_name.to_owned()],
    };

    for name in names {
        let path = upload_dir.join(&name);
        match std::fs::remove_file(&path) {
            Ok(()) => info!("Deleted media: {}", path.display()),
            Err(err) if err.kind() == ErrorKind::NotFound => {}
            Err(err) => warn!("Failed to delete {}: {}", path.display(), err),
        }
    }
}
