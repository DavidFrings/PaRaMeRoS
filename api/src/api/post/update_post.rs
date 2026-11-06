use actix_web::{
    HttpRequest, HttpResponse, put,
    web::{Data, Json, Path},
};
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use super::section::{normalize_section, validate_season};
use crate::db::schema::posts::updated_at;
use crate::db::{
    models::UpdatePostRequest,
    schema::posts::{
        dsl::{content, content_en, heading, heading_en, name, posts, season},
        uuid as uuid_schema,
    },
};
use crate::utils::{HttpError, bad_request, db, verify};
use crate::{Env, query};

macro_rules! update {
    ($content:expr, $new_content:expr, $uuid:expr, $conn:expr) => {
        query!(
            diesel::update(posts.filter(uuid_schema.eq($uuid)))
                .set($content.eq($new_content))
                .execute($conn)
        )
    };
}

#[put("/post/{post_id}")]
pub async fn update_post(
    req: HttpRequest,
    path: Path<String>,
    update_data: Json<UpdatePostRequest>,
    env: Data<Env>,
) -> Result<HttpResponse, HttpError> {
    verify(&req, &env).await?;

    let data = update_data.into_inner();
    let req_path = path.into_inner();
    let req_uuid = Uuid::parse_str(&req_path)
        .map_err(|err| bad_request(format!("Invalid UUID format: {}", err)))?;

    if let Some(s) = data.season.as_deref().filter(|s| !s.is_empty()) {
        validate_season(s)?;
    }

    let mut conn = db(&env).await?;
    let mut was_updated = false;

    if let Some(new_name) = data.name {
        let new_name = normalize_section(&new_name)?;
        update!(name, new_name, &req_uuid, &mut conn);
        was_updated = true;
    }
    if let Some(new_heading) = data.heading {
        update!(heading, new_heading, &req_uuid, &mut conn);
        was_updated = true;
    }
    if let Some(new_content) = data.content {
        update!(content, new_content, &req_uuid, &mut conn);
        was_updated = true;
    }

    // An empty string clears the optional field to NULL.
    let opt = |v: String| Some(v).filter(|v| !v.is_empty());
    if let Some(v) = data.heading_en {
        update!(heading_en, opt(v), &req_uuid, &mut conn);
        was_updated = true;
    }
    if let Some(v) = data.content_en {
        update!(content_en, opt(v), &req_uuid, &mut conn);
        was_updated = true;
    }
    if let Some(v) = data.season {
        update!(season, opt(v), &req_uuid, &mut conn);
        was_updated = true;
    }

    if was_updated {
        update!(updated_at, Utc::now().naive_utc(), &req_uuid, &mut conn);
    }

    Ok(HttpResponse::Accepted().finish())
}
