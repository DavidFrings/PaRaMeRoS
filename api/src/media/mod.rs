pub mod backfill;
pub mod files;
pub mod image;
pub mod video;

use crate::utils::{HttpError, bad_request, internal_error, payload_too_large};
use actix_multipart::{Field, Multipart};
use actix_web::rt::task::spawn_blocking;
use futures::StreamExt;
use std::{collections::HashMap, path::PathBuf};
use uuid::Uuid;

pub const MAX_IMAGE_BYTES: usize = 15 * 1024 * 1024;
pub const MAX_VIDEO_BYTES: usize = 100 * 1024 * 1024;
pub const MAX_TEXT_BYTES: usize = 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoFormat {
    Mp4,
    Webm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaKind {
    Image,
    Video(VideoFormat),
}

/// Read a multipart field, failing with 413 as soon as more than `max` bytes arrived.
pub async fn read_limited(field: &mut Field, max: usize) -> Result<Vec<u8>, HttpError> {
    let mut content = Vec::new();
    while let Some(chunk) = field.next().await {
        let bytes =
            chunk.map_err(|err| internal_error(format!("Failed to read content: {err}")))?;
        if content.len() + bytes.len() > max {
            return Err(payload_too_large(format!("Field exceeds {max} bytes")));
        }
        content.extend_from_slice(&bytes);
    }
    Ok(content)
}

/// Detect the media kind from magic bytes; only JPEG, PNG, WebP, MP4 and WebM are accepted.
pub fn sniff(bytes: &[u8]) -> Result<MediaKind, HttpError> {
    use ::image::ImageFormat::{Jpeg, Png, WebP};
    if matches!(::image::guess_format(bytes), Ok(Jpeg | Png | WebP)) {
        Ok(MediaKind::Image)
    } else if bytes.get(4..8) == Some(b"ftyp") {
        Ok(MediaKind::Video(VideoFormat::Mp4))
    } else if bytes.starts_with(&[0x1A, 0x45, 0xDF, 0xA3]) {
        Ok(MediaKind::Video(VideoFormat::Webm))
    } else {
        Err(bad_request("Unsupported media format"))
    }
}

/// Text fields and the (single) file of a multipart form.
pub struct Form {
    pub text: HashMap<String, String>,
    pub file: Option<Vec<u8>>,
}

/// Read a multipart form. Fields with a filename are the media file; other names must be in `allowed`.
pub async fn read_form(payload: &mut Multipart, allowed: &[&str]) -> Result<Form, HttpError> {
    let mut form = Form {
        text: HashMap::new(),
        file: None,
    };

    while let Some(item) = payload.next().await {
        let mut field =
            item.map_err(|err| internal_error(format!("Failed to read Multipart: {err}")))?;
        let disposition = field
            .content_disposition()
            .ok_or_else(|| bad_request("Missing Content-Disposition header!"))?;
        let name = disposition
            .get_name()
            .ok_or_else(|| bad_request("Field name missing!"))?
            .to_owned();

        if disposition.get_filename().is_some() {
            // Cut images off early when the type is already declared; the exact per-kind
            // limit is checked again in `store_upload`.
            let max = match form.text.get("media_type").map(|t| t.to_lowercase()) {
                Some(t) if t == "img" => MAX_IMAGE_BYTES,
                _ => MAX_VIDEO_BYTES,
            };
            form.file = Some(read_limited(&mut field, max).await?);
        } else if allowed.contains(&name.as_str()) {
            let text = String::from_utf8(read_limited(&mut field, MAX_TEXT_BYTES).await?)
                .map_err(|err| bad_request(format!("Invalid UTF-8: {err}")))?;
            form.text.insert(name, text);
        } else {
            return Err(bad_request(format!("Unexpected field: {name}")));
        }
    }

    Ok(form)
}

/// Validate and store an uploaded file for `post`. Returns `(media_type, media_name)` for the DB.
pub async fn store_upload(
    upload_dir: PathBuf,
    post: Uuid,
    media_type: &str,
    data: Vec<u8>,
) -> Result<(String, String), HttpError> {
    let media_type = media_type.to_lowercase();
    let max = match media_type.as_str() {
        "img" => MAX_IMAGE_BYTES,
        "vid" => MAX_VIDEO_BYTES,
        _ => return Err(bad_request(format!("Invalid media type: {media_type}"))),
    };
    if data.len() > max {
        return Err(payload_too_large(format!("Media exceeds {max} bytes")));
    }

    let kind = sniff(&data)?;
    let base = format!("{post}-{}", &Uuid::new_v4().simple().to_string()[..8]);

    let name = match (media_type.as_str(), kind) {
        ("img", MediaKind::Image) => {
            spawn_blocking(move || image::store_image(&upload_dir, &base, &data))
                .await
                .map_err(|err| internal_error(format!("Image task failed: {err}")))??
        }
        ("vid", MediaKind::Video(fmt)) => video::store_video(&upload_dir, &base, fmt, &data)?,
        _ => return Err(bad_request("File does not match media_type")),
    };

    Ok((media_type, name))
}
