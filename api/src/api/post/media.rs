use crate::db::{
    models::{MediaResponse, Post},
    schema::posts::{
        dsl::{media_creator, media_name, media_type, posts, updated_at},
        uuid as uuid_schema,
    },
};
use crate::media::{files::remove_media_files, read_form, store_upload};
use crate::redis_store::cache_clear_posts;
use crate::utils::{HttpError, bad_request, db, require, verify};
use crate::{Env, query};
use actix_multipart::Multipart;
use actix_web::{
    HttpRequest, HttpResponse, delete, put,
    web::{Data, Path},
};
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

async fn find_post(env: &Data<Env>, id: &Uuid) -> Result<Post, HttpError> {
    let mut conn = db(env).await?;
    Ok(query!(
        posts
            .filter(uuid_schema.eq(id))
            .select(Post::as_select())
            .first(&mut conn)
    ))
}

fn parse_uuid(path: Path<String>) -> Result<Uuid, HttpError> {
    Uuid::parse_str(&path.into_inner())
        .map_err(|err| bad_request(format!("Invalid UUID format: {err}")))
}

#[put("/post/{post_id}/media")]
pub async fn replace_media(
    req: HttpRequest,
    path: Path<String>,
    mut payload: Multipart,
    env: Data<Env>,
) -> Result<HttpResponse, HttpError> {
    let user = verify(&req, &env).await?;
    let id = parse_uuid(path)?;
    let old = find_post(&env, &id).await?;
    require(&user, &old.name)?;

    let mut form = read_form(&mut payload, &["media_type", "media_creator"]).await?;
    let kind = form
        .text
        .remove("media_type")
        .ok_or_else(|| bad_request("media_type missing!"))?;
    let data = form
        .file
        .ok_or_else(|| bad_request("Media file missing!"))?;
    // DB CHECK media_fields needs a creator with the media; fail before any file is written.
    let creator = form
        .text
        .remove("media_creator")
        .filter(|c| !c.trim().is_empty())
        .ok_or_else(|| bad_request("Media creator missing!"))?;
    let (new_type, new_name) = store_upload(env.upload_dir.clone(), id, &kind, data).await?;

    let mut conn = db(&env).await?;
    query!(
        diesel::update(posts.filter(uuid_schema.eq(&id)))
            .set((
                media_type.eq(&new_type),
                media_name.eq(&new_name),
                media_creator.eq(creator),
                updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(&mut conn)
    );

    if let Some(old_name) = old.media_name {
        remove_media_files(&env.upload_dir, &old_name);
    }

    cache_clear_posts(&env).await;

    Ok(HttpResponse::Ok().json(MediaResponse {
        media_name: new_name,
    }))
}

#[delete("/post/{post_id}/media")]
pub async fn delete_media(
    req: HttpRequest,
    path: Path<String>,
    env: Data<Env>,
) -> Result<HttpResponse, HttpError> {
    let user = verify(&req, &env).await?;
    let id = parse_uuid(path)?;
    let old = find_post(&env, &id).await?;
    require(&user, &old.name)?;

    let mut conn = db(&env).await?;
    query!(
        diesel::update(posts.filter(uuid_schema.eq(&id)))
            .set((
                media_type.eq(None::<String>),
                media_name.eq(None::<String>),
                media_creator.eq(None::<String>),
                updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(&mut conn)
    );

    if let Some(old_name) = old.media_name {
        remove_media_files(&env.upload_dir, &old_name);
    }

    cache_clear_posts(&env).await;

    Ok(HttpResponse::NoContent().finish())
}
