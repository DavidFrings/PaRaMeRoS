use diesel::prelude::*;
use diesel::SelectableHelper;
use serde::{Deserialize, Serialize};
use crate::db::schema::{users, events};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: usize
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Event {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub img: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = events)]
pub struct NewEvent {
    pub title: String,
    pub description: String,
    pub img: Option<String>,
}