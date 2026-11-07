use crate::db::{
    models::{Post, PostResponse},
    schema::posts::{dsl::posts, uuid as uuid_schema},
};
use crate::redis_store::{cache_get, cache_set};
use crate::utils::{HttpError, bad_request, db, internal_error};
use crate::{Env, query};
use actix_web::{
    HttpResponse, get,
    web::{Data, Path},
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

#[get("/post/{post_id}")]
pub async fn post(path: Path<String>, env: Data<Env>) -> Result<HttpResponse, HttpError> {
    let req_path = path.into_inner();
    let req_uuid = Uuid::parse_str(&req_path)
        .map_err(|err| bad_request(format!("Invalid UUID format: {}", err)))?;

    let key = format!("cache:post:{req_uuid}");
    if let Some(body) = cache_get(&env, &key).await {
        return Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body));
    }

    let mut conn = db(&env).await?;

    let post = query!(
        posts
            .filter(uuid_schema.eq(&req_uuid))
            .limit(1)
            .select(Post::as_select())
            .get_result(&mut conn)
    );

    let json = PostResponse::from(post);

    let body = serde_json::to_string(&json)
        .map_err(|err| internal_error(format!("Serialization failed: {err}")))?;
    cache_set(&env, &key, &body, 300).await;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}
