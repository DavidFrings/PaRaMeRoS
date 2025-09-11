use crate::db::{
    models::{Post, PostResponse},
    schema::posts::{dsl::posts as posts_dsl, name},
};
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
    let req_path = path.into_inner();

    let mut conn = db(&env).await?;

    let posts = query!(
        posts_dsl
            .filter(name.eq(&req_path))
            .get_results::<Post>(&mut conn)
    );

    let json: Vec<PostResponse> = posts
        .into_iter()
        .map(|post| PostResponse::from(post))
        .collect();

    Ok(HttpResponse::Ok().json(json))
}
