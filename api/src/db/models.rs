use crate::db::schema::{posts, users};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
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
    pub admin: bool,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub admin: bool,
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
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
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
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

//*****************************//
//* Request / Response Models *//
//*****************************//

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub admin: bool,
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
}

#[derive(Serialize, Debug)]
pub struct AuthVerifyResponse {
    pub admin: bool,
}

#[derive(Serialize, Debug)]
pub struct NewPostResponse {
    pub id: Uuid,
}

#[derive(Serialize, Debug)]
pub struct PostResponse {
    pub uuid: Uuid,
    pub name: String,
    pub heading: String,
    pub content: String,
    pub media_type: Option<String>,
    pub media_name: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
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
            created_at: post.created_at,
            updated_at: post.updated_at,
        }
    }
}
