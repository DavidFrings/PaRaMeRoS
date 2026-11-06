mod common;

use actix_web::{
    dev::{Service, ServiceResponse},
    http::header::AUTHORIZATION,
    test::{TestRequest, call_service, read_body_json},
};
use image::{ImageBuffer, ImageFormat, Rgb, RgbImage};
use serde_json::Value;
use std::{io::Cursor, path::Path};

const MP4: [u8; 12] = [
    0x00, 0x00, 0x00, 0x18, 0x66, 0x74, 0x79, 0x70, 0x69, 0x73, 0x6F, 0x6D,
];
const WEBM: [u8; 4] = [0x1A, 0x45, 0xDF, 0xA3];

fn padded(head: &[u8], len: usize) -> Vec<u8> {
    let mut v = head.to_vec();
    v.resize(len, 0);
    v
}

fn encoded(w: u32, h: u32, fmt: ImageFormat) -> Vec<u8> {
    let img: RgbImage =
        ImageBuffer::from_fn(w, h, |x, y| Rgb([(x % 256) as u8, (y % 256) as u8, 128]));
    let mut out = Cursor::new(Vec::new());
    img.write_to(&mut out, fmt).expect("encode fixture");
    out.into_inner()
}

fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &b in data {
        crc ^= b as u32;
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ 0xEDB8_8320
            } else {
                crc >> 1
            };
        }
    }
    !crc
}

fn png_chunk(out: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
    out.extend((data.len() as u32).to_be_bytes());
    let mut body = kind.to_vec();
    body.extend(data);
    out.extend(&body);
    out.extend(crc32(&body).to_be_bytes());
}

/// Valid PNG header claiming 8000x6000 with a bogus IDAT: must be rejected before decoding.
fn huge_png_header() -> Vec<u8> {
    let mut out = vec![0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
    let mut ihdr = Vec::new();
    ihdr.extend(8000u32.to_be_bytes());
    ihdr.extend(6000u32.to_be_bytes());
    ihdr.extend([8, 2, 0, 0, 0]);
    png_chunk(&mut out, b"IHDR", &ihdr);
    png_chunk(&mut out, b"IDAT", &[0, 0, 0, 0]);
    png_chunk(&mut out, b"IEND", &[]);
    out
}

fn multipart(fields: &[(&str, &str)], file: Option<(&str, &[u8])>) -> Vec<u8> {
    let mut body = Vec::new();
    for (k, v) in fields {
        body.extend(
            format!("--XB\r\nContent-Disposition: form-data; name=\"{k}\"\r\n\r\n{v}\r\n")
                .as_bytes(),
        );
    }
    if let Some((filename, data)) = file {
        body.extend(
            format!(
                "--XB\r\nContent-Disposition: form-data; name=\"media\"; filename=\"{filename}\"\r\nContent-Type: application/octet-stream\r\n\r\n"
            )
            .as_bytes(),
        );
        body.extend(data);
        body.extend(b"\r\n");
    }
    body.extend(b"--XB--\r\n");
    body
}

async fn send<S, B>(app: &S, req: TestRequest, auth: &str, body: Vec<u8>) -> ServiceResponse<B>
where
    S: Service<actix_http::Request, Response = ServiceResponse<B>, Error = actix_web::Error>,
{
    let req = req
        .insert_header((AUTHORIZATION, auth))
        .insert_header(("Content-Type", "multipart/form-data; boundary=XB"))
        .set_payload(body)
        .to_request();
    call_service(app, req).await
}

/// Create a post with the given media; returns the response.
async fn create<S, B>(
    app: &S,
    auth: &str,
    media_type: &str,
    file: (&str, &[u8]),
) -> ServiceResponse<B>
where
    S: Service<actix_http::Request, Response = ServiceResponse<B>, Error = actix_web::Error>,
{
    let body = multipart(
        &[
            ("name", "team"),
            ("heading", "H"),
            ("content", "C"),
            ("media_type", media_type),
            ("media_creator", "me"),
        ],
        Some(file),
    );
    send(app, TestRequest::post().uri("/post"), auth, body).await
}

async fn get_post<S, B>(app: &S, id: &str) -> Value
where
    S: Service<actix_http::Request, Response = ServiceResponse<B>, Error = actix_web::Error>,
    B: actix_web::body::MessageBody,
{
    let res = call_service(
        app,
        TestRequest::get().uri(&format!("/post/{id}")).to_request(),
    )
    .await;
    assert_eq!(res.status(), 200);
    read_body_json(res).await
}

/// Create a post and return (uuid, media_name).
async fn create_ok<S, B>(
    app: &S,
    auth: &str,
    media_type: &str,
    file: (&str, &[u8]),
) -> (String, String)
where
    S: Service<actix_http::Request, Response = ServiceResponse<B>, Error = actix_web::Error>,
    B: actix_web::body::MessageBody,
{
    let res = create(app, auth, media_type, file).await;
    assert_eq!(res.status(), 200);
    let body: Value = read_body_json(res).await;
    let id = body["id"].as_str().unwrap().to_owned();
    let post = get_post(app, &id).await;
    (id, post["media_name"].as_str().unwrap().to_owned())
}

fn variants(dir: &Path, media_name: &str) -> Vec<std::path::PathBuf> {
    let base = media_name.strip_suffix(".webp").expect("webp name");
    ["webp", "jxl", "800.webp", "800.jxl"]
        .iter()
        .map(|s| match s.strip_prefix("800.") {
            Some(ext) => dir.join(format!("{base}-800.{ext}")),
            None => dir.join(format!("{base}.{s}")),
        })
        .collect()
}

#[actix_web::test]
async fn jpeg_upload_creates_four_variants() {
    let (app, env) = common::test_app().await;
    let auth = common::login(&app).await;
    let (id, name) = create_ok(
        &app,
        &auth,
        "img",
        ("photo.jpg", &encoded(1600, 1200, ImageFormat::Jpeg)),
    )
    .await;

    assert!(name.starts_with(&format!("{id}-")), "{name}");
    let files = variants(&env.upload_dir, &name);
    for f in &files {
        assert!(f.exists(), "{}", f.display());
    }
    assert_eq!(image::image_dimensions(&files[2]).unwrap().0, 800);
    let jxl = std::fs::read(&files[1]).unwrap();
    assert!(
        jxl.starts_with(&[0xFF, 0x0A]) || jxl.starts_with(&[0, 0, 0, 0x0C, 0x4A, 0x58, 0x4C, 0x20]),
        "not a JPEG XL file"
    );
}

#[actix_web::test]
async fn small_image_not_upscaled() {
    let (app, env) = common::test_app().await;
    let auth = common::login(&app).await;
    let (_, name) = create_ok(
        &app,
        &auth,
        "img",
        ("p.png", &encoded(400, 300, ImageFormat::Png)),
    )
    .await;
    let files = variants(&env.upload_dir, &name);
    assert_eq!(image::image_dimensions(&files[2]).unwrap().0, 400);
}

#[actix_web::test]
async fn oversized_image_returns_413() {
    let (app, _) = common::test_app().await;
    let auth = common::login(&app).await;
    let data = padded(&[0xFF, 0xD8, 0xFF, 0xE0], 15 * 1024 * 1024 + 1);
    let res = create(&app, &auth, "img", ("big.jpg", &data)).await;
    assert_eq!(res.status(), 413);
}

#[actix_web::test]
async fn fake_jpeg_returns_400() {
    let (app, _) = common::test_app().await;
    let auth = common::login(&app).await;
    let res = create(
        &app,
        &auth,
        "img",
        ("x.jpg", b"just some text, not an image"),
    )
    .await;
    assert_eq!(res.status(), 400);
}

#[actix_web::test]
async fn too_many_pixels_returns_400() {
    let (app, env) = common::test_app().await;
    let auth = common::login(&app).await;
    let res = create(&app, &auth, "img", ("huge.png", &huge_png_header())).await;
    assert_eq!(res.status(), 400);
    // The IDAT is garbage, so this message proves the dimension check ran before decoding.
    let body = actix_web::test::read_body(res).await;
    assert!(
        String::from_utf8_lossy(&body).contains("8000x6000"),
        "{body:?}"
    );
    assert_eq!(std::fs::read_dir(&env.upload_dir).unwrap().count(), 0);
}

#[actix_web::test]
async fn webm_video_stored_as_webm() {
    let (app, env) = common::test_app().await;
    let auth = common::login(&app).await;
    let data = padded(&WEBM, 1024);
    let (id, name) = create_ok(&app, &auth, "vid", ("clip.mp4", &data)).await;
    assert!(
        name.starts_with(&format!("{id}-")) && name.ends_with(".webm"),
        "{name}"
    );
    assert_eq!(std::fs::read(env.upload_dir.join(&name)).unwrap(), data);
    assert_eq!(get_post(&app, &id).await["media_type"], "vid");
}

#[actix_web::test]
async fn mp4_video_stored_as_mp4() {
    let (app, env) = common::test_app().await;
    let auth = common::login(&app).await;
    let (_, name) = create_ok(&app, &auth, "vid", ("clip.mov", &padded(&MP4, 1024))).await;
    assert!(name.ends_with(".mp4"), "{name}");
    assert!(env.upload_dir.join(&name).exists());
}

#[actix_web::test]
async fn oversized_video_returns_413() {
    let (app, _) = common::test_app().await;
    let auth = common::login(&app).await;
    let data = padded(&MP4, 100 * 1024 * 1024 + 1);
    let res = create(&app, &auth, "vid", ("big.mp4", &data)).await;
    assert_eq!(res.status(), 413);
}

#[actix_web::test]
async fn oversized_text_field_returns_413() {
    let (app, _) = common::test_app().await;
    let auth = common::login(&app).await;
    let content = "a".repeat(1024 * 1024 + 1);
    let body = multipart(
        &[("name", "team"), ("heading", "H"), ("content", &content)],
        None,
    );
    let res = send(&app, TestRequest::post().uri("/post"), &auth, body).await;
    assert_eq!(res.status(), 413);
}

#[actix_web::test]
async fn media_type_mismatch_returns_400() {
    let (app, _) = common::test_app().await;
    let auth = common::login(&app).await;
    let res = create(&app, &auth, "img", ("a.mp4", &padded(&MP4, 1024))).await;
    assert_eq!(res.status(), 400);
}

#[actix_web::test]
async fn replace_media_keeps_uuid_and_removes_old_files() {
    let (app, env) = common::test_app().await;
    let auth = common::login(&app).await;
    let (id, old) = create_ok(
        &app,
        &auth,
        "img",
        ("a.jpg", &encoded(1600, 1200, ImageFormat::Jpeg)),
    )
    .await;

    let body = multipart(
        &[("media_type", "img"), ("media_creator", "other")],
        Some(("b.png", &encoded(400, 300, ImageFormat::Png))),
    );
    let res = send(
        &app,
        TestRequest::put().uri(&format!("/post/{id}/media")),
        &auth,
        body,
    )
    .await;
    assert_eq!(res.status(), 200);
    let json: Value = read_body_json(res).await;
    let new = json["media_name"].as_str().unwrap().to_owned();
    assert_ne!(new, old);

    for f in variants(&env.upload_dir, &old) {
        assert!(!f.exists(), "{}", f.display());
    }
    for f in variants(&env.upload_dir, &new) {
        assert!(f.exists(), "{}", f.display());
    }
    let post = get_post(&app, &id).await;
    assert_eq!(post["uuid"], id.as_str());
    assert_eq!(post["media_name"], new.as_str());
    assert_eq!(post["media_creator"], "other");
    assert!(!post["updated_at"].is_null());
}

#[actix_web::test]
async fn replace_media_requires_auth_and_known_post() {
    let (app, _) = common::test_app().await;
    let auth = common::login(&app).await;
    let file = encoded(10, 10, ImageFormat::Png);
    let uri = format!("/post/{}/media", uuid::Uuid::new_v4());
    let body = multipart(&[("media_type", "img")], Some(("a.png", &file)));
    let res = send(
        &app,
        TestRequest::put().uri(&uri),
        "Bearer nope",
        body.clone(),
    )
    .await;
    assert_eq!(res.status(), 401);
    let res = send(&app, TestRequest::put().uri(&uri), &auth, body).await;
    assert_eq!(res.status(), 404);
    let res = send(&app, TestRequest::delete().uri(&uri), &auth, Vec::new()).await;
    assert_eq!(res.status(), 404);
}

#[actix_web::test]
async fn delete_media_clears_columns() {
    let (app, env) = common::test_app().await;
    let auth = common::login(&app).await;
    let (id, name) = create_ok(
        &app,
        &auth,
        "img",
        ("a.jpg", &encoded(1600, 1200, ImageFormat::Jpeg)),
    )
    .await;

    let res = send(
        &app,
        TestRequest::delete().uri(&format!("/post/{id}/media")),
        &auth,
        Vec::new(),
    )
    .await;
    assert_eq!(res.status(), 204);
    for f in variants(&env.upload_dir, &name) {
        assert!(!f.exists(), "{}", f.display());
    }
    let post = get_post(&app, &id).await;
    assert!(post["media_type"].is_null());
    assert!(post["media_name"].is_null());
    assert!(post["media_creator"].is_null());
}

#[actix_web::test]
async fn delete_post_removes_all_image_variants_and_video() {
    let (app, env) = common::test_app().await;
    let auth = common::login(&app).await;

    let (id, name) = create_ok(
        &app,
        &auth,
        "img",
        ("a.jpg", &encoded(1600, 1200, ImageFormat::Jpeg)),
    )
    .await;
    let files = variants(&env.upload_dir, &name);
    assert!(files.iter().all(|f| f.exists()));
    let res = send(
        &app,
        TestRequest::delete().uri(&format!("/post/{id}")),
        &auth,
        Vec::new(),
    )
    .await;
    assert!(res.status().is_success());
    assert!(files.iter().all(|f| !f.exists()));
    let res = call_service(
        &app,
        TestRequest::get().uri(&format!("/post/{id}")).to_request(),
    )
    .await;
    assert_eq!(res.status(), 404);

    let (id, name) = create_ok(&app, &auth, "vid", ("v.webm", &padded(&WEBM, 64))).await;
    assert!(env.upload_dir.join(&name).exists());
    let res = send(
        &app,
        TestRequest::delete().uri(&format!("/post/{id}")),
        &auth,
        Vec::new(),
    )
    .await;
    assert!(res.status().is_success());
    assert!(!env.upload_dir.join(&name).exists());
}

#[actix_web::test]
async fn backfill_creates_missing_variants() {
    let dir = tempfile::tempdir().unwrap();
    let img = image::load_from_memory(&encoded(1000, 500, ImageFormat::Png))
        .unwrap()
        .to_rgb8();
    let webp = webp::Encoder::from_rgb(&img, 1000, 500).encode(80.0);
    std::fs::write(dir.path().join("legacy.webp"), &*webp).unwrap();
    std::fs::write(dir.path().join("clip.mp4"), padded(&MP4, 64)).unwrap();

    parameros_api::media::backfill::backfill(dir.path().to_path_buf()).await;

    let mut names: Vec<String> = std::fs::read_dir(dir.path())
        .unwrap()
        .map(|e| e.unwrap().file_name().into_string().unwrap())
        .collect();
    names.sort();
    assert_eq!(
        names,
        [
            "clip.mp4",
            "legacy-800.jxl",
            "legacy-800.webp",
            "legacy.jxl",
            "legacy.webp"
        ]
    );
    assert_eq!(
        image::image_dimensions(dir.path().join("legacy-800.webp")).unwrap(),
        (800, 400)
    );
}
