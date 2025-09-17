use crate::Env;
use crate::db::{
    models::{Claims, User},
    schema::users::{dsl::users, id},
};
use actix_web::{
    HttpRequest, HttpResponse, ResponseError,
    http::{StatusCode, header::AUTHORIZATION},
    web::Data,
};
use derive_more::Display;
use diesel::prelude::*;
use diesel_async::{
    AsyncPgConnection, RunQueryDsl, pooled_connection::AsyncDieselConnectionManager,
};
use jsonwebtoken::{DecodingKey, Validation, decode};

#[macro_export]
macro_rules! query {
    ($query:expr) => {
        $query
            .await
            .map_err(|err| internal_error(format!("Failed to find req in the db: {}", err)))?
    };
}

#[derive(Debug, Display)]
#[display("{}", message)]
pub struct HttpError {
    message: String,
    status: StatusCode,
}

impl HttpError {
    pub fn new(msg: impl Into<String>, status: StatusCode) -> Self {
        Self {
            message: msg.into(),
            status,
        }
    }
}

impl ResponseError for HttpError {
    fn status_code(&self) -> StatusCode {
        self.status
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status).body(self.message.clone())
    }
}

pub fn bad_request(msg: impl Into<String>) -> HttpError {
    HttpError::new(msg, StatusCode::BAD_REQUEST)
}

pub fn _bad_gateway(msg: impl Into<String>) -> HttpError {
    HttpError::new(msg, StatusCode::BAD_GATEWAY)
}

pub fn internal_error(msg: impl Into<String>) -> HttpError {
    HttpError::new(msg, StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn unauthorized(msg: impl Into<String>) -> HttpError {
    HttpError::new(msg, StatusCode::UNAUTHORIZED)
}

pub fn forbidden(msg: impl Into<String>) -> HttpError {
    HttpError::new(msg, StatusCode::FORBIDDEN)
}

pub async fn verify(req: &HttpRequest, env: &Data<Env>) -> Result<i32, HttpError> {
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| return bad_request("Missing Authorization header!"))?;

    let token = auth_header.replace("Bearer ", "");

    let req_id: i32 = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(env.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|err| return unauthorized(format!("Invalid token: {}", err)))?
    .claims
    .sub
    .parse()
    .map_err(|err| return internal_error(format!("Failed to parse id: {}", err)))?;

    let mut conn = db(&env).await?;

    query!(
        users
            .filter(id.eq(&req_id))
            .limit(1)
            .get_result::<User>(&mut conn)
    );

    Ok(req_id)
}

pub async fn db(
    env: &Data<Env>,
) -> Result<bb8::PooledConnection<AsyncDieselConnectionManager<AsyncPgConnection>>, HttpError> {
    let conn = env
        .pool
        .get()
        .await
        .map_err(|err| return internal_error(format!("Database connection error: {}", err)))?;
    Ok(conn)
}
