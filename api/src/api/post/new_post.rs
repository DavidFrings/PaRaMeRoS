use super::section::{normalize_section, validate_season};
use crate::db::{
    models::{NewPost, NewPostResponse, Post},
    schema::posts::dsl::posts,
};
use crate::media::{read_form, store_upload};
use crate::redis_store::cache_clear_posts;
use crate::utils::{HttpError, bad_request, db, forbidden, require, verify};
use crate::{Env, query};
use actix_multipart::Multipart;
use actix_web::{HttpRequest, HttpResponse, post, web::Data};
use chrono::Utc;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use log::info;
use uuid::Uuid;

#[post("/post")]
pub async fn new_post(
    req: HttpRequest,
    mut payload: Multipart,
    env: Data<Env>,
) -> Result<HttpResponse, HttpError> {
    let user = verify(&req, &env).await?;
    // Refuse before reading a possibly large upload; the exact section is checked below.
    let sections = ["logbuch", "team", "coaches", "forest-project"];
    if !sections.iter().any(|s| require(&user, s).is_ok()) {
        return Err(forbidden("Missing permission"));
    }

    let mut form = read_form(
        &mut payload,
        &[
            "name",
            "heading",
            "content",
            "media_type",
            "media_creator",
            "heading_en",
            "content_en",
            "season",
        ],
    )
    .await?;
    let mut take = |key: &str| form.text.remove(key);
    let non_empty = |v: Option<String>| v.filter(|t| !t.is_empty());

    let name = normalize_section(&take("name").ok_or_else(|| bad_request("Name missing!"))?)?;
    require(&user, name)?;
    let season = non_empty(take("season"));
    if let Some(season) = &season {
        validate_season(season)?;
    }
    let heading = take("heading").ok_or_else(|| bad_request("Heading missing!"))?;
    let content = take("content").ok_or_else(|| bad_request("Content missing!"))?;
    let heading_en = non_empty(take("heading_en"));
    let content_en = non_empty(take("content_en"));
    let creator = take("media_creator").filter(|c| !c.trim().is_empty());
    let post_uuid = Uuid::new_v4();

    // A creator without media is ignored; media without a creator violates the DB CHECK.
    let (db_media_type, db_media_name, media_creator) = match (take("media_type"), form.file) {
        (Some(media_type), Some(data)) => {
            let creator = creator.ok_or_else(|| bad_request("Media creator missing!"))?;
            let (t, n) = store_upload(env.upload_dir.clone(), post_uuid, &media_type, data).await?;
            (Some(t), Some(n), Some(creator))
        }
        _ => (None, None, None),
    };

    let new_post = NewPost {
        uuid: post_uuid,
        name: name.to_string(),
        heading,
        content,
        media_type: db_media_type,
        media_name: db_media_name,
        media_creator,
        created_at: Utc::now().naive_utc(),
        updated_at: None,
        heading_en,
        content_en,
        season,
    };

    let mut conn = db(&env).await?;

    query!(
        diesel::insert_into(posts)
            .values(&new_post)
            .returning(Post::as_returning())
            .get_result(&mut conn)
    );

    cache_clear_posts(&env).await;
    info!("Created new post {} on {}", new_post.heading, new_post.name);

    Ok(HttpResponse::Ok().json(NewPostResponse { id: new_post.uuid }))
}
