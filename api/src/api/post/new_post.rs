use crate::db::{
    models::{NewPost, NewPostResponse, Post},
    schema::posts::dsl::posts,
};
use crate::utils::{HttpError, bad_request, db, internal_error, verify};
use crate::{Env, UPLOAD_DIR, query};
use actix_multipart::{Field, Multipart};
use actix_web::{HttpRequest, HttpResponse, post, web::Data};
use chrono::Utc;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use futures::{AsyncWriteExt, StreamExt};
use log::info;
use std::fs;
use uuid::Uuid;
use webp::{Encoder, WebPMemory};

async fn process_field(field: &mut Field) -> Result<Vec<u8>, HttpError> {
    let mut content = Vec::new();

    while let Some(chunk) = field.next().await {
        let bytes = chunk
            .map_err(|err| return internal_error(format!("Failed to read content: {}", err)))?;
        content
            .write_all(&bytes)
            .await
            .map_err(|err| return internal_error(format!("Failed to process content: {}", err)))?;
    }

    Ok(content)
}

async fn to_webp(data: &[u8]) -> Result<Vec<u8>, HttpError> {
    let img = image::load_from_memory(data)
        .map_err(|err| return internal_error(format!("Failed to load image: {}", err)))?;
    let encoder = Encoder::from_image(&img)
        .map_err(|err| return internal_error(format!("Failed to encode: {}", err)))?;
    let webp: WebPMemory = encoder.encode(80.0);

    Ok(webp.to_vec())
}

async fn save_media(name: &String, data: Vec<u8>) -> Result<(), HttpError> {
    if name.contains("..") || name.contains('/') || name.contains('\\') {
        return Err(bad_request("Invalid filename!"));
    }

    let file_path = format!("{}/{}", UPLOAD_DIR, name);

    fs::write(&file_path, data)
        .map_err(|err| return internal_error(format!("Failed to save media: {}", err)))?;

    info!("Saved media: {}", file_path);
    Ok(())
}

#[post("/post")]
pub async fn new_post(
    req: HttpRequest,
    mut payload: Multipart,
    env: Data<Env>,
) -> Result<HttpResponse, HttpError> {
    verify(&req, &env).await?;

    let mut name = None;
    let mut heading = None;
    let mut content = None;
    let mut media_type = None;
    let mut media_name = None;
    let mut media_data = None;
    let mut media_creator = None;

    while let Some(item) = payload.next().await {
        let mut field = item
            .map_err(|err| return internal_error(format!("Failed to read Multipart: {}", err)))?;

        let content_disposition = field
            .content_disposition()
            .ok_or_else(|| return bad_request("Missing Content-Disposition header!"))?;
        let field_name = content_disposition
            .get_name()
            .ok_or_else(|| return bad_request("Field name missing!"))?
            .to_string();
        let filename = content_disposition.get_filename().map(|f| f.to_owned());

        if let Some(filename) = filename {
            let data = process_field(&mut field).await?;

            media_name = Some(format!(
                "{}-{}",
                Utc::now().format("%d%m%Y-%H%M%S"),
                filename.to_owned()
            ));
            media_data = Some(data);
        } else {
            let text_content = process_field(&mut field).await?;

            let text = String::from_utf8(text_content)
                .map_err(|err| return bad_request(format!("Invalid UTF-8: {}", err)))?;

            match field_name.as_str() {
                "name" => name = Some(text),
                "heading" => heading = Some(text),
                "content" => content = Some(text),
                "media_type" => media_type = Some(text),
                "media_creator" => media_creator = Some(text),
                _ => return Err(bad_request(format!("Unexpected field: {}", field_name))),
            }
        }
    }

    let name = name.ok_or_else(|| bad_request("Name missing!"))?;
    let heading = heading.ok_or_else(|| bad_request("Heading missing!"))?;
    let content = content.ok_or_else(|| bad_request("Content missing!"))?;
    let mut db_media_type = None;
    let mut db_media_name = None;

    if let (Some(media_type), Some(media_name), Some(media_data)) = (media_type, media_name, media_data) {
        match media_type.to_lowercase().as_str() {
            "img" => {
                let webp_data = to_webp(&media_data).await?;

                let webp_name = if media_name.contains('.') {
                    let parts: Vec<&str> = media_name.rsplitn(2, '.').collect();
                    format!("{}.webp", parts[1])
                } else {
                    format!("{}.webp", media_name)
                };

                save_media(&webp_name, webp_data).await?;

                db_media_type = Some("img".to_string());
                db_media_name = Some(webp_name);
            },
            "vid" => {

                let mp4_name = if media_name.contains('.') {
                    let parts: Vec<&str> = media_name.rsplitn(2, '.').collect();
                    format!("{}.mp4", parts[1])
                } else {
                    format!("{}.mp4", media_name)
                };

                save_media(&mp4_name, media_data).await?;

                db_media_type = Some("vid".to_string());
                db_media_name = Some(mp4_name);
            },
            _ => return Err(bad_request(format!("Invalid media type: {}", media_type))),
        }
    }

    let new_post = NewPost {
        uuid: Uuid::new_v4(),
        name,
        heading,
        content,
        media_type: db_media_type,
        media_name: db_media_name,
        media_creator,
        created_at: Utc::now().naive_utc(),
        updated_at: None,
    };

    let mut conn = db(&env).await?;

    query!(
        diesel::insert_into(posts)
            .values(&new_post)
            .returning(Post::as_returning())
            .get_result(&mut conn)
    );

    info!("Created new post {} on {}", new_post.heading, new_post.name);

    Ok(HttpResponse::Ok().json(NewPostResponse { id: new_post.uuid }))
}
