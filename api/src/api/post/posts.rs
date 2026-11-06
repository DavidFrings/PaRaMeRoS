use super::section::normalize_section;
use crate::db::{
    models::{Post, PostResponse},
    schema::posts::{created_at, dsl::posts as posts_dsl, name},
};
use crate::utils::{HttpError, db};
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

    let mut conn = db(&env).await?;

    let posts = query!(
        posts_dsl
            .filter(name.eq(section))
            .order(created_at.desc())
            .select(Post::as_select())
            .get_results(&mut conn)
    );

    let json: Vec<PostResponse> = posts.into_iter().map(PostResponse::from).collect();

    Ok(HttpResponse::Ok().json(json))
}
