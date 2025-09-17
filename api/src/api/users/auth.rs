use crate::Env;
use crate::db::{
    models::{Claims, LoginRequest, User},
    schema::users::{dsl::users, username},
};
use crate::utils::{HttpError, db, internal_error, unauthorized};
use actix_web::{
    HttpResponse,
    http::header::{ACCESS_CONTROL_EXPOSE_HEADERS, AUTHORIZATION},
    post,
    web::{Data, Json},
};
use bcrypt::verify;
use chrono::{Duration, Utc};
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use jsonwebtoken::{EncodingKey, Header, encode};
use log::info;

#[post("/auth")]
pub async fn auth(
    login_data: Json<LoginRequest>,
    env: Data<Env>,
) -> Result<HttpResponse, HttpError> {
    let data = login_data.into_inner();

    let mut conn = db(&env).await?;

    // Verify username
    let usr = users
        .filter(username.eq(&data.username))
        .limit(1)
        .get_result::<User>(&mut conn)
        .await
        .map_err(|_| return unauthorized("Invalid username or password"))?;

    // Verify password
    let password_matches = verify(&data.password, &usr.password).map_err(|err| {
        return internal_error(format!("Password verification error: {}", err));
    })?;

    if !password_matches {
        return Err(unauthorized("Invalid username or password"));
    }

    info!("User logged in: {}", usr.username);

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
    .map_err(|err| return internal_error(format!("Could not create a token: {}", err)))?;

    Ok(HttpResponse::Ok()
        .insert_header((AUTHORIZATION, format!("Bearer {}", token)))
        .insert_header((ACCESS_CONTROL_EXPOSE_HEADERS, "authorization"))
        .finish())
}
