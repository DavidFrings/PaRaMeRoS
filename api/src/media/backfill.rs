use super::image::{MAX_PIXELS, write_variants};
use actix_web::rt::task::spawn_blocking;
use log::{error, info, warn};
use std::path::{Path, PathBuf};

/// Create missing JPEG XL / 800 px variants for legacy `*.webp` uploads. Never panics.
pub async fn backfill(upload_dir: PathBuf) {
    // A panic inside the blocking task surfaces as a JoinError and is only logged.
    match spawn_blocking(move || run(&upload_dir)).await {
        Ok((created, skipped, failed)) => {
            info!("Media backfill: created {created}, skipped {skipped}, failed {failed}")
        }
        Err(err) => error!("Media backfill task failed: {err}"),
    }
}

fn run(upload_dir: &Path) -> (usize, usize, usize) {
    let (mut created, mut skipped, mut failed) = (0, 0, 0);
    let entries = match std::fs::read_dir(upload_dir) {
        Ok(entries) => entries,
        Err(err) => {
            warn!(
                "Media backfill: cannot read {}: {err}",
                upload_dir.display()
            );
            return (0, 0, 0);
        }
    };

    for entry in entries.flatten() {
        let file_name = entry.file_name();
        let Some(base) = file_name
            .to_str()
            .and_then(|n| n.strip_suffix(".webp"))
            .filter(|b| !b.ends_with("-800"))
        else {
            continue;
        };

        let complete = [".jxl", "-800.webp", "-800.jxl"]
            .iter()
            .all(|suffix| upload_dir.join(format!("{base}{suffix}")).exists());
        if complete {
            skipped += 1;
            continue;
        }

        // Header-only check: a huge legacy file must not OOM the API on every boot.
        match image::ImageReader::open(entry.path())
            .and_then(|r| r.into_dimensions().map_err(std::io::Error::other))
        {
            Ok((w, h)) if u64::from(w) * u64::from(h) > MAX_PIXELS => {
                warn!("Media backfill: {base}.webp skipped, {w}x{h} exceeds MAX_PIXELS");
                skipped += 1;
                continue;
            }
            _ => {}
        }

        match image::open(entry.path())
            .map_err(|err| err.to_string())
            .and_then(|img| write_variants(upload_dir, base, &img, true))
        {
            Ok(n) => created += n,
            Err(err) => {
                warn!("Media backfill: {base}.webp failed: {err}");
                failed += 1;
            }
        }
    }
    (created, skipped, failed)
}
