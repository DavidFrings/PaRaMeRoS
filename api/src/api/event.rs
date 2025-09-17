use actix_web::{post, HttpResponse, HttpRequest, web::Data};
use actix_multipart::{Field, Multipart};
use diesel_async::RunQueryDsl;
use diesel::SelectableHelper;
use futures::{StreamExt, TryStreamExt};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::path::Path;
use uuid::Uuid;
use webp::Encoder;
use crate::db::{models::{Claims, NewEvent, Event}, schema::events::dsl::events};
use crate::Env;
use crate::utils::{internal_error, unauthorized, HttpError};

async fn process_field(mut field: Field) -> Result<String, HttpError> {
    let mut content = Vec::new();

    while let Some(chunk) = field.next().await {
        content.extend_from_slice(&chunk
            .map_err(|_|
                internal_error("Error reading title!")
            )?
        );
    }

    String::from_utf8(content)
        .map_err(|_|
            internal_error("Invalid UTF-8 text!")
        )
}

#[post("/event")]
pub async fn new_event(req: HttpRequest, mut payload: Multipart, env: Data<Env>) -> Result<HttpResponse, HttpError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| unauthorized("Missing Authorization header!"))?;

    let token = auth_header.replace("Bearer ", "");

    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(env.jwt_secret.as_bytes()),
        &Validation::default()
    ).map_err(|err|
        unauthorized(format!("Invalid token: {}", err))
    )?;

    let id = Uuid::new_v4();
    let mut title = None;
    let mut description = None;
    let mut img_filename = None;

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field
            .content_disposition()
            .ok_or_else(|| internal_error("Content-Disposition header missing!"))?;
        let field_name = content_disposition
            .get_name()
            .unwrap_or("");

        match field_name {
            "title" => {
                title = Some(process_field(field).await?);
            },
            "description" => {
                description = Some(process_field(field).await?);
            },
            "img" => {
                // 0 Bytes Error
                let mut img_data = Vec::new();
                let mut webp_data = Vec::new();

                while let Some(chunk) = field.next().await {
                    img_data.extend_from_slice(&chunk
                        .map_err(|_|
                            internal_error("Error reading title!")
                        )?
                    );
                }

                let dyn_image = image::load_from_memory(&img_data)
                    .map_err(|err|
                        internal_error(format!("Failed to decode image: {}", err))
                    )?;

                let encoder = Encoder::from_image(&dyn_image).map_err(|err| {
                    internal_error(format!("Failed to convert image to WebP: {}", err))
                })?;
                let encoded = encoder.encode_lossless();
                webp_data.extend_from_slice(&encoded);

                std::fs::create_dir_all("uploads")
                    .map_err(|err|
                        internal_error(format!("Failed to create an upload directory: {}", err))
                    )?;

                let filename = format!("{}.webp", id);
                let filepath = Path::new("uploads").join(&filename);

                let _file = std::fs::File::create(filepath)
                    .map_err(|err|
                        internal_error(format!("Failed to create the image: {}", err))
                    )?;

                img_filename = Some(filename);
        },
            _ => {
                internal_error("To many fields!");
            }
        }
    }

    let title = title.ok_or_else(||
        internal_error("Title missing!")
    )?;
    let description = description.ok_or_else(||
        internal_error("Description missing!")
    )?;

    let new_event = NewEvent {
        title,
        description,
        img: img_filename,
    };

    let mut conn = env.pool.get()
        .await
        .map_err(|err|
            internal_error(format!("Database connection error: {}", err))
        )?;

    let res = diesel::insert_into(events)
        .values(&new_event)
        .returning(Event::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(|err|
            internal_error(format!("Error creating event: {}", err))
        )?;

    Ok(HttpResponse::Ok().json(res))
}