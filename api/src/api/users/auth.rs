use crate::Env;
use crate::client_ip::{client_ip, rate_limit_key_ip};
use crate::db::{
    models::{Claims, LoginRequest, User},
    schema::users::{dsl::users, username},
};
use crate::redis_store::{check_limit, hit_limit, reset_limit};
use crate::utils::{HttpError, db, internal_error, too_many_requests, unauthorized};
use actix_web::{
    HttpRequest, HttpResponse,
    http::header::{ACCESS_CONTROL_EXPOSE_HEADERS, AUTHORIZATION},
    post,
    web::{Data, Json},
};
use bcrypt::verify;
use chrono::{Duration, Utc};
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use jsonwebtoken::{EncodingKey, Header, encode};
use log::{error, info};

// bcrypt hash at DEFAULT_COST (12) so unknown users cost as much as real ones
const DUMMY_HASH: &str = "$2b$12$iEGsObAItIyS3tRrXZXu3uH5g7cIFZl.bcy4DmIj5C0le..4D3wRm";

const LOGIN_MAX_FAILURES: u32 = 5;
const LOGIN_WINDOW_SECS: u64 = 900;

#[post("/auth")]
pub async fn auth(
    req: HttpRequest,
    login_data: Json<LoginRequest>,
    env: Data<Env>,
) -> Result<HttpResponse, HttpError> {
    let data = login_data.into_inner();

    // Only failed attempts count; checked before the expensive bcrypt work.
    let limit_key = format!("rl:login:{}", rate_limit_key_ip(client_ip(&req)));
    if let Some(retry) = check_limit(&env, &limit_key, LOGIN_MAX_FAILURES).await {
        return Err(too_many_requests(retry));
    }

    let mut conn = db(&env).await?;

    // Verify username
    let usr = match users
        .filter(username.eq(&data.username))
        .limit(1)
        .get_result::<User>(&mut conn)
        .await
    {
        Ok(usr) => usr,
        Err(diesel::result::Error::NotFound) => {
            // Equalise timing with the real-user path to prevent username enumeration
            let _ = verify(&data.password, DUMMY_HASH);
            hit_limit(&env, &limit_key, LOGIN_MAX_FAILURES, LOGIN_WINDOW_SECS).await;
            return Err(unauthorized("Invalid username or password"));
        }
        Err(err) => {
            error!("Database error: {}", err);
            return Err(internal_error("Database error"));
        }
    };

    // Verify password
    let password_matches = verify(&data.password, &usr.password).map_err(|err| {
        error!("Password verification error: {}", err);
        internal_error("Authentication failed")
    })?;

    if !password_matches {
        hit_limit(&env, &limit_key, LOGIN_MAX_FAILURES, LOGIN_WINDOW_SECS).await;
        return Err(unauthorized("Invalid username or password"));
    }

    reset_limit(&env, &limit_key).await;
    info!("User logged in");

    // JWT token
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(env.jwt_expire))
        .expect("Invalid timestamp!")
        .timestamp() as usize;

    let claims = Claims {
        sub: usr.id.to_string(),
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(env.jwt_secret.as_bytes()),
    )
    .map_err(|err| {
        error!("Could not create token: {}", err);
        internal_error("Authentication failed")
    })?;

    Ok(HttpResponse::Ok()
        .insert_header((AUTHORIZATION, format!("Bearer {}", token)))
        .insert_header((ACCESS_CONTROL_EXPOSE_HEADERS, "authorization"))
        .finish())
}
