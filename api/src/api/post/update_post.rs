use actix_web::{
    HttpRequest, HttpResponse, put,
    web::{Data, Json, Path},
};
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::{AsyncConnection, RunQueryDsl, scoped_futures::ScopedFutureExt};
use uuid::Uuid;

use super::section::{normalize_section, validate_season};
use crate::Env;
use crate::db::schema::posts::updated_at;
use crate::db::{
    models::UpdatePostRequest,
    schema::posts::{
        dsl::{content, content_en, heading, heading_en, name, posts, season},
        uuid as uuid_schema,
    },
};
use crate::redis_store::cache_clear_posts;
use crate::utils::{HttpError, bad_request, db, require, verify};

macro_rules! update {
    ($content:expr, $new_content:expr, $uuid:expr, $conn:expr) => {
        diesel::update(posts.filter(uuid_schema.eq($uuid)))
            .set($content.eq($new_content))
            .execute($conn)
            .await?
    };
}

#[put("/post/{post_id}")]
pub async fn update_post(
    req: HttpRequest,
    path: Path<String>,
    update_data: Json<UpdatePostRequest>,
    env: Data<Env>,
) -> Result<HttpResponse, HttpError> {
    let user = verify(&req, &env).await?;

    let data = update_data.into_inner();
    let req_path = path.into_inner();
    let req_uuid = Uuid::parse_str(&req_path)
        .map_err(|err| bad_request(format!("Invalid UUID format: {err}")))?;

    if let Some(s) = data.season.as_deref().filter(|s| !s.is_empty()) {
        validate_season(s)?;
    }

    let new_name = data.name.as_deref().map(normalize_section).transpose()?;

    // Declared before `conn`: the transaction borrows them until the pooled connection drops.
    let uuid = &req_uuid;
    let data = &data;
    let user = &user;
    let mut conn = db(&env).await?;

    // One transaction: a failing statement must not leave a half-applied update.
    conn.transaction::<_, HttpError, _>(|conn| {
        async move {
            let mut was_updated = false;
            // Locked row: the section checked is the one written. 404 comes before 403.
            let current: String = posts
                .filter(uuid_schema.eq(uuid))
                .select(name)
                .for_update()
                .first(conn)
                .await?;
            require(user, &current)?;
            // Moving a post also writes to the new section.
            if let Some(new_name) = new_name {
                require(user, new_name)?;
            }
            if let Some(new_name) = new_name {
                update!(name, new_name, uuid, conn);
                was_updated = true;
            }
            if let Some(v) = &data.heading {
                update!(heading, v, uuid, conn);
                was_updated = true;
            }
            if let Some(v) = &data.content {
                update!(content, v, uuid, conn);
                was_updated = true;
            }

            // An empty string clears the optional field to NULL.
            let opt = |v: &String| Some(v.clone()).filter(|v| !v.is_empty());
            if let Some(v) = &data.heading_en {
                update!(heading_en, opt(v), uuid, conn);
                was_updated = true;
            }
            if let Some(v) = &data.content_en {
                update!(content_en, opt(v), uuid, conn);
                was_updated = true;
            }
            if let Some(v) = &data.season {
                update!(season, opt(v), uuid, conn);
                was_updated = true;
            }

            if was_updated {
                update!(updated_at, Utc::now().naive_utc(), uuid, conn);
            }
            Ok(())
        }
        .scope_boxed()
    })
    .await?;

    cache_clear_posts(&env).await;
    Ok(HttpResponse::Accepted().finish())
}
