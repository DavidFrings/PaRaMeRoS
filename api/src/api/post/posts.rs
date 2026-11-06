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
    let req_path = path.into_inner();

    let mut conn = db(&env).await?;

    let posts = query!(
        posts_dsl
            .filter(name.eq(&req_path))
            .order(created_at.desc())
            .get_results::<Post>(&mut conn)
    );

    let json: Vec<PostResponse> = posts.into_iter().map(PostResponse::from).collect();

    Ok(HttpResponse::Ok().json(json))
}
