use crate::db::{
    models::Post,
    schema::posts::{dsl::posts, uuid as uuid_schema},
};
use crate::utils::{HttpError, bad_request, db, internal_error, verify};
use crate::{Env, UPLOAD_DIR, query};
use actix_web::{
    HttpRequest, HttpResponse, delete,
    web::{Data, Path},
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use log::info;
use std::fs;
use uuid::Uuid;

#[delete("/post/{post_id}")]
pub async fn delete_post(
    req: HttpRequest,
    path: Path<String>,
    env: Data<Env>,
) -> Result<HttpResponse, HttpError> {
    verify(&req, &env).await?;

    let req_path = path.into_inner();
    let req_uuid = Uuid::parse_str(&req_path)
        .map_err(|err| return bad_request(format!("Invalid UUID format: {}", err)))?;

    let mut conn = db(&env).await?;

    let post = query!(
        posts
            .filter(uuid_schema.eq(&req_uuid))
            .limit(1)
            .get_result::<Post>(&mut conn)
    );

    if let Some(media_name) = post.media_name {
        let file_path = format!("{}/{}", UPLOAD_DIR, media_name);

        fs::remove_file(&file_path)
            .map_err(|err| return internal_error(format!("Failed to delete image: {}", err)))?;
        info!("Deleted image: {}", file_path);
    }

    query!(diesel::delete(posts.filter(uuid_schema.eq(&req_uuid))).execute(&mut conn));

    info!("Deleted post {} on {}", post.heading, post.name);

    Ok(HttpResponse::Accepted().finish())
}
