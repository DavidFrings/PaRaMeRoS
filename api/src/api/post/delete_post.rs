use crate::Env;
use crate::db::{
    models::Post,
    schema::posts::{dsl::posts, uuid as uuid_schema},
};
use crate::media::files::remove_media_files;
use crate::redis_store::cache_clear_posts;
use crate::utils::{HttpError, bad_request, db, require, verify};
use actix_web::{
    HttpRequest, HttpResponse, delete,
    web::{Data, Path},
};
use diesel::prelude::*;
use diesel_async::{AsyncConnection, RunQueryDsl, scoped_futures::ScopedFutureExt};
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
        .map_err(|err| bad_request(format!("Invalid UUID format: {err}")))?;

    let mut conn = db(&env).await?;

    let uuid = &req_uuid;
    let user = &user;
    let post = conn
        .transaction::<_, HttpError, _>(|conn| {
            async move {
                // Locked row: the section checked is the one deleted. 404 comes before 403.
                let post = posts
                    .filter(uuid_schema.eq(uuid))
                    .select(Post::as_select())
                    .for_update()
                    .first(conn)
                    .await?;
                require(user, &post.name)?;
                diesel::delete(posts.filter(uuid_schema.eq(uuid)))
                    .execute(conn)
                    .await?;
                Ok(post)
            }
            .scope_boxed()
        })
        .await?;

    if let Some(media_name) = post.media_name {
        remove_media_files(&env.upload_dir, &media_name);
    }

    cache_clear_posts(&env).await;
    info!("Deleted post");

    Ok(HttpResponse::Accepted().finish())
}
