use crate::Env;
use crate::db::{
    models::{Claims, User},
    schema::users::{dsl::users, id},
};
use actix_web::{
    HttpRequest, HttpResponse, ResponseError,
    http::{
        StatusCode,
        header::{AUTHORIZATION, RETRY_AFTER},
    },
    web::Data,
};
use derive_more::Display;
use diesel::prelude::*;
use diesel_async::{
    AsyncPgConnection, RunQueryDsl, pooled_connection::AsyncDieselConnectionManager,
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use log::{error, warn};

#[macro_export]
macro_rules! query {
    ($query:expr) => {
        $query.await.map_err(|err| match err {
            diesel::result::Error::NotFound => $crate::utils::not_found("Not found"),
            err => {
                log::error!("Database error: {}", err);
                $crate::utils::internal_error("Database error")
            }
        })?
    };
}

#[derive(Debug, Display)]
#[display("{}", message)]
pub struct HttpError {
    message: String,
    status: StatusCode,
    retry_after: Option<u64>,
}

impl HttpError {
    pub fn new(msg: impl Into<String>, status: StatusCode) -> Self {
        Self {
            message: msg.into(),
            status,
            retry_after: None,
        }
    }
}

impl ResponseError for HttpError {
    fn status_code(&self) -> StatusCode {
        self.status
    }

    fn error_response(&self) -> HttpResponse {
        let mut res = HttpResponse::build(self.status);
        if let Some(secs) = self.retry_after {
            res.insert_header((RETRY_AFTER, secs.to_string()));
        }
        res.body(self.message.clone())
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

pub fn not_found(msg: impl Into<String>) -> HttpError {
    HttpError::new(msg, StatusCode::NOT_FOUND)
}

pub fn payload_too_large(msg: impl Into<String>) -> HttpError {
    HttpError::new(msg, StatusCode::PAYLOAD_TOO_LARGE)
}

pub fn unauthorized(msg: impl Into<String>) -> HttpError {
    HttpError::new(msg, StatusCode::UNAUTHORIZED)
}

pub fn forbidden(msg: impl Into<String>) -> HttpError {
    HttpError::new(msg, StatusCode::FORBIDDEN)
}

pub fn too_many_requests(retry_after_secs: u64) -> HttpError {
    HttpError {
        retry_after: Some(retry_after_secs),
        ..HttpError::new("Too many requests", StatusCode::TOO_MANY_REQUESTS)
    }
}

pub async fn verify(req: &HttpRequest, env: &Data<Env>) -> Result<i32, HttpError> {
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| bad_request("Missing Authorization header!"))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| unauthorized("Invalid token"))?;

    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    let req_id: i32 = decode::<Claims>(
        token,
        &DecodingKey::from_secret(env.jwt_secret.as_bytes()),
        &validation,
    )
    .map_err(|err| {
        warn!("Token verification failed: {}", err);
        unauthorized("Invalid token")
    })?
    .claims
    .sub
    .parse()
    .map_err(|err| {
        warn!("Token subject parsing failed: {}", err);
        unauthorized("Invalid token")
    })?;

    let mut conn = db(env).await?;

    users
        .filter(id.eq(&req_id))
        .limit(1)
        .get_result::<User>(&mut conn)
        .await
        .map_err(|err| match err {
            diesel::result::Error::NotFound => unauthorized("Invalid token"),
            err => {
                error!("Database error: {}", err);
                internal_error("Database error")
            }
        })?;

    Ok(req_id)
}

pub async fn db(
    env: &Data<Env>,
) -> Result<bb8::PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>>, HttpError> {
    let conn = env.pool.get().await.map_err(|err| {
        error!("Database connection error: {}", err);
        internal_error("Database error")
    })?;
    Ok(conn)
}
