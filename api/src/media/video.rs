use super::VideoFormat;
use crate::utils::{HttpError, internal_error};
use log::info;
use std::path::Path;

/// Store a sniffed video as `<base>.mp4` or `<base>.webm`. Videos are not re-encoded.
pub fn store_video(
    upload_dir: &Path,
    base: &str,
    fmt: VideoFormat,
    bytes: &[u8],
) -> Result<String, HttpError> {
    let name = match fmt {
        VideoFormat::Mp4 => format!("{base}.mp4"),
        VideoFormat::Webm => format!("{base}.webm"),
    };
    let path = upload_dir.join(&name);
    std::fs::write(&path, bytes)
        .map_err(|err| internal_error(format!("Failed to save media: {err}")))?;
    info!("Saved media: {}", path.display());
    Ok(name)
}
