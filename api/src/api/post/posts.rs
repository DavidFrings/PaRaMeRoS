use super::section::normalize_section;
use crate::db::{
    models::{Post, PostResponse},
    schema::posts::{created_at, dsl::posts as posts_dsl, name},
};
use crate::redis_store::{cache_get, cache_set};
use crate::utils::{HttpError, db, internal_error};
use crate::{Env, query};
use actix_web::{
    HttpResponse, get,
    web::{Data, Path},
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

#[get("/posts/{name}")]
pub async fn posts(path: Path<String>, env: Data<Env>) -> Result<HttpResponse, HttpError> {
    let section = normalize_section(&path.into_inner())?;

    let key = format!("cache:posts:{section}");
    if let Some(body) = cache_get(&env, &key).await {
        return Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body));
    }

    let mut conn = db(&env).await?;

    let posts = query!(
        posts_dsl
            .filter(name.eq(section))
            .order(created_at.desc())
            .select(Post::as_select())
            .get_results(&mut conn)
    );

    let json: Vec<PostResponse> = posts.into_iter().map(PostResponse::from).collect();

    let body = serde_json::to_string(&json)
        .map_err(|err| internal_error(format!("Serialization failed: {err}")))?;
    cache_set(&env, &key, &body, 300).await;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}
