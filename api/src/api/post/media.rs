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
use diesel_async::{
    AsyncConnection, AsyncPgConnection, RunQueryDsl, scoped_futures::ScopedFutureExt,
};
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

/// The post row, locked until the transaction ends, so the section checked is the one written.
async fn locked_post(conn: &mut AsyncPgConnection, id: &Uuid) -> Result<Post, HttpError> {
    Ok(posts
        .filter(uuid_schema.eq(id))
        .select(Post::as_select())
        .for_update()
        .first(conn)
        .await?)
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
    // Re-checked under the row lock below; this check keeps forbidden uploads off disk.
    require(&user, &find_post(&env, &id).await?.name)?;

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

    // Owned copy: the transaction's borrows live as long as the pooled connection.
    let (user, id, new_type, stored) = (&user, &id, &new_type, new_name.clone());
    let mut conn = db(&env).await?;
    let res = conn
        .transaction::<_, HttpError, _>(|conn| {
            async move {
                let old = locked_post(conn, id).await?;
                require(user, &old.name)?;
                diesel::update(posts.filter(uuid_schema.eq(id)))
                    .set((
                        media_type.eq(new_type),
                        media_name.eq(stored),
                        media_creator.eq(creator),
                        updated_at.eq(Utc::now().naive_utc()),
                    ))
                    .execute(conn)
                    .await?;
                Ok(old.media_name)
            }
            .scope_boxed()
        })
        .await;
    let old_name = res.inspect_err(|_| remove_media_files(&env.upload_dir, &new_name))?;

    if let Some(old_name) = old_name {
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

    // Declared before `conn`: the transaction borrows them until the pooled connection drops.
    let (user, id) = (&user, &id);
    let mut conn = db(&env).await?;
    let old_name = conn
        .transaction::<_, HttpError, _>(|conn| {
            async move {
                let old = locked_post(conn, id).await?;
                require(user, &old.name)?;
                diesel::update(posts.filter(uuid_schema.eq(id)))
                    .set((
                        media_type.eq(None::<String>),
                        media_name.eq(None::<String>),
                        media_creator.eq(None::<String>),
                        updated_at.eq(Utc::now().naive_utc()),
                    ))
                    .execute(conn)
                    .await?;
                Ok(old.media_name)
            }
            .scope_boxed()
        })
        .await?;

    if let Some(old_name) = old_name {
        remove_media_files(&env.upload_dir, &old_name);
    }

    cache_clear_posts(&env).await;

    Ok(HttpResponse::NoContent().finish())
}
