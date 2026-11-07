use super::files::remove_media_files;
use crate::utils::{HttpError, bad_request, internal_error};
use image::{DynamicImage, ImageReader, imageops::FilterType};
use jpegxl_rs::encode::{EncoderFrame, EncoderSpeed, encoder_builder};
use log::info;
use std::{io::Cursor, path::Path};

pub const MAX_PIXELS: u64 = 40_000_000;
pub const SMALL_WIDTH: u32 = 800;

/// Validate the dimensions (before decoding), decode and write all four variants.
/// Returns `"<base>.webp"`.
pub fn store_image(upload_dir: &Path, base: &str, bytes: &[u8]) -> Result<String, HttpError> {
    let reader = || {
        ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()
            .map_err(|err| bad_request(format!("Invalid image: {err}")))
    };
    let (w, h) = reader()?
        .into_dimensions()
        .map_err(|err| bad_request(format!("Invalid image: {err}")))?;
    if u64::from(w) * u64::from(h) > MAX_PIXELS {
        return Err(bad_request(format!("Image too large: {w}x{h} pixels")));
    }
    let img = reader()?
        .decode()
        .map_err(|err| bad_request(format!("Invalid image: {err}")))?;

    let name = format!("{base}.webp");
    if let Err(err) = write_variants(upload_dir, base, &img, false) {
        remove_media_files(upload_dir, &name);
        return Err(internal_error(err));
    }
    info!("Saved media: {}", upload_dir.join(&name).display());
    Ok(name)
}

/// Write `<base>.webp`, `<base>.jxl`, `<base>-800.webp` and `<base>-800.jxl`.
/// With `only_missing`, existing files are kept. Returns the number of files written.
pub fn write_variants(
    upload_dir: &Path,
    base: &str,
    img: &DynamicImage,
    only_missing: bool,
) -> Result<usize, String> {
    let small = (img.width() > SMALL_WIDTH)
        .then(|| img.resize(SMALL_WIDTH, u32::MAX, FilterType::Lanczos3));
    let small = small.as_ref().unwrap_or(img);

    let variants: [(String, &DynamicImage, Option<f32>); 4] = [
        (format!("{base}.webp"), img, None),
        (format!("{base}.jxl"), img, Some(1.0)),
        (format!("{base}-800.webp"), small, None),
        (format!("{base}-800.jxl"), small, Some(1.5)),
    ];

    let mut written = 0;
    for (name, img, jxl_distance) in variants {
        let path = upload_dir.join(&name);
        if only_missing && path.exists() {
            continue;
        }
        let data = match jxl_distance {
            Some(distance) => to_jxl(img, distance)?,
            None => to_webp(img)?,
        };
        std::fs::write(&path, data).map_err(|err| format!("Failed to save {name}: {err}"))?;
        written += 1;
    }
    Ok(written)
}

fn to_webp(img: &DynamicImage) -> Result<Vec<u8>, String> {
    // The webp encoder only takes RGB(A)8, so normalise grey/16-bit images first.
    let img = if img.color().has_alpha() {
        DynamicImage::ImageRgba8(img.to_rgba8())
    } else {
        DynamicImage::ImageRgb8(img.to_rgb8())
    };
    let encoder = webp::Encoder::from_image(&img).map_err(|err| format!("WebP: {err}"))?;
    Ok(encoder.encode(80.0).to_vec())
}

fn to_jxl(img: &DynamicImage, distance: f32) -> Result<Vec<u8>, String> {
    let has_alpha = img.color().has_alpha();
    let mut encoder = encoder_builder()
        .speed(EncoderSpeed::Squirrel) // effort 7
        .quality(distance)
        .has_alpha(has_alpha)
        .build()
        .map_err(|err| format!("JPEG XL: {err}"))?;
    let (pixels, channels) = if has_alpha {
        (img.to_rgba8().into_raw(), 4)
    } else {
        (img.to_rgb8().into_raw(), 3)
    };
    encoder
        .encode_frame(
            &EncoderFrame::new(&pixels).num_channels(channels),
            img.width(),
            img.height(),
        )
        .map_err(|err| format!("JPEG XL: {err}"))
}
