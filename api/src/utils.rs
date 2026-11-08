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

/// Diesel errors inside transactions: `NotFound` is a 404, anything else a logged 500.
impl From<diesel::result::Error> for HttpError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => not_found("Not found"),
            err => {
                error!("Database error: {err}");
                internal_error("Database error")
            }
        }
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

#[must_use]
pub fn too_many_requests(retry_after_secs: u64) -> HttpError {
    HttpError {
        retry_after: Some(retry_after_secs),
        ..HttpError::new("Too many requests", StatusCode::TOO_MANY_REQUESTS)
    }
}

/// Every permission value; section names equal post `name` values.
pub const PERMISSIONS: [&str; 5] = ["logbuch", "team", "coaches", "forest-project", "tournament"];

/// Admins hold every permission; everyone else only what `permissions` lists.
///
/// # Errors
/// 403 if the user lacks `perm`.
pub fn require(user: &User, perm: &str) -> Result<(), HttpError> {
    if user.admin || user.permissions.iter().flatten().any(|p| p == perm) {
        Ok(())
    } else {
        Err(forbidden("Missing permission"))
    }
}

/// `verify` plus a permission check: 401 without a valid token, 403 without `perm`.
///
/// # Errors
/// 400/401 like `verify`, 403 without `perm`, 500 on database errors.
pub async fn authorize(req: &HttpRequest, env: &Data<Env>, perm: &str) -> Result<i32, HttpError> {
    let user = verify(req, env).await?;
    require(&user, perm)?;
    Ok(user.id)
}

/// Validate the `Authorization: Bearer <JWT>` header and load its user.
///
/// # Errors
/// 400 without the header, 401 for an invalid token or unknown user, 500 on database errors.
pub async fn verify(req: &HttpRequest, env: &Data<Env>) -> Result<User, HttpError> {
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
        warn!("Token verification failed: {err}");
        unauthorized("Invalid token")
    })?
    .claims
    .sub
    .parse()
    .map_err(|err| {
        warn!("Token subject parsing failed: {err}");
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
                error!("Database error: {err}");
                internal_error("Database error")
            }
        })
}

/// Get a pooled database connection.
///
/// # Errors
/// 500 if no connection is available.
pub async fn db(
    env: &Data<Env>,
) -> Result<bb8::PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>>, HttpError> {
    let conn = env.pool.get().await.map_err(|err| {
        error!("Database connection error: {err}");
        internal_error("Database error")
    })?;
    Ok(conn)
}
