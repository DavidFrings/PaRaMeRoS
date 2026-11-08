use crate::db::schema::{posts, tournament, users};
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

//*****************************//
//*         DB Models         *//
//*****************************//

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    /// `normal`, `forest` or `super` (DB CHECK); see `utils::role_allows`.
    pub role: String,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    /// `normal`, `forest` or `super` (DB CHECK); see `utils::role_allows`.
    pub role: String,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub uuid: Uuid,
    pub name: String,
    pub heading: String,
    pub content: String,
    pub media_type: Option<String>,
    pub media_name: Option<String>,
    pub media_creator: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub heading_en: Option<String>,
    pub content_en: Option<String>,
    pub season: Option<String>,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub uuid: Uuid,
    pub name: String,
    pub heading: String,
    pub content: String,
    pub media_type: Option<String>,
    pub media_name: Option<String>,
    pub media_creator: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub heading_en: Option<String>,
    pub content_en: Option<String>,
    pub season: Option<String>,
}

//*****************************//
//* Request / Response Models *//
//*****************************//

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    /// Defaults to `normal`; checked against `ROLES`.
    pub role: Option<String>,
    pub admin_pass: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct UpdatePostRequest {
    pub name: Option<String>,
    pub heading: Option<String>,
    pub content: Option<String>,
    pub heading_en: Option<String>,
    pub content_en: Option<String>,
    pub season: Option<String>,
    /// RFC 3339 with offset; `super` only. Not a content edit, so it alone leaves `updated_at`.
    pub created_at: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct AuthVerifyResponse {
    pub role: String,
    pub permissions: Vec<&'static str>,
    pub can_set_created_at: bool,
}

#[derive(Serialize, Debug)]
pub struct NewPostResponse {
    pub id: Uuid,
}

#[derive(Serialize, Debug)]
pub struct MediaResponse {
    pub media_name: String,
}

#[derive(Serialize, Debug)]
pub struct PostResponse {
    pub uuid: Uuid,
    pub name: String,
    pub heading: String,
    pub content: String,
    pub media_type: Option<String>,
    pub media_name: Option<String>,
    pub media_creator: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub heading_en: Option<String>,
    pub content_en: Option<String>,
    pub season: Option<String>,
}

impl From<Post> for PostResponse {
    fn from(post: Post) -> Self {
        PostResponse {
            uuid: post.uuid,
            name: post.name,
            heading: post.heading,
            content: post.content,
            media_type: post.media_type,
            media_name: post.media_name,
            media_creator: post.media_creator,
            created_at: post.created_at,
            updated_at: post.updated_at,
            heading_en: post.heading_en,
            content_en: post.content_en,
            season: post.season,
        }
    }
}

#[derive(Queryable, Selectable, Insertable, AsChangeset, Serialize, Debug)]
#[diesel(table_name = tournament)]
#[diesel(treat_none_as_null = true)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tournament {
    pub name: String,
    pub place: Option<String>,
    pub starts_at: DateTime<Utc>,
    pub ends_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
}
