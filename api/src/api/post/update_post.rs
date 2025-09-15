use actix_web::{
    HttpRequest, HttpResponse, put,
    web::{Data, Json, Path},
};
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::db::schema::posts::updated_at;
use crate::db::{
    models::UpdatePostRequest,
    schema::posts::{
        dsl::{content, heading, name, posts},
        uuid as uuid_schema,
    },
};
use crate::utils::{HttpError, bad_request, db, internal_error, verify};
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
        .map_err(|err| return bad_request(format!("Invalid UUID format: {}", err)))?;

    let mut conn = db(&env).await?;
    let mut was_updated = false;

    if let Some(new_name) = data.name {
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

    if was_updated {
        update!(updated_at, Utc::now().naive_utc(), &req_uuid, &mut conn);
    }

    Ok(HttpResponse::Accepted().finish())
}
