use crate::db::{
    models::Post,
    schema::posts::{dsl::posts, uuid as uuid_schema},
};
use crate::media::files::remove_media_files;
use crate::redis_store::cache_clear_posts;
use crate::utils::{HttpError, bad_request, db, require, verify};
use crate::{Env, query};
use actix_web::{
    HttpRequest, HttpResponse, delete,
    web::{Data, Path},
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use log::info;
use uuid::Uuid;

#[delete("/post/{post_id}")]
pub async fn delete_post(
    req: HttpRequest,
    path: Path<String>,
    env: Data<Env>,
) -> Result<HttpResponse, HttpError> {
    let user = verify(&req, &env).await?;

    let req_path = path.into_inner();
    let req_uuid = Uuid::parse_str(&req_path)
        .map_err(|err| bad_request(format!("Invalid UUID format: {}", err)))?;

    let mut conn = db(&env).await?;

    let post = query!(
        posts
            .filter(uuid_schema.eq(&req_uuid))
            .limit(1)
            .select(Post::as_select())
            .get_result(&mut conn)
    );
    require(&user, &post.name)?;

    query!(diesel::delete(posts.filter(uuid_schema.eq(&req_uuid))).execute(&mut conn));

    if let Some(media_name) = post.media_name {
        remove_media_files(&env.upload_dir, &media_name);
    }

    cache_clear_posts(&env).await;
    info!("Deleted post {} on {}", post.heading, post.name);

    Ok(HttpResponse::Accepted().finish())
}
