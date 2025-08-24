use actix_web::{post, web::{Data, Json, block}, HttpResponse};
use bcrypt::verify;
use chrono::{Duration, Utc};
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};
use jsonwebtoken::{encode, EncodingKey, Header};
use crate::db::{models::{AuthResponse, Claims, LoginRequest, User}, schema::users::{dsl::users, name}};
use crate::Env;
use crate::utils::{internal_error, unauthorized, HttpError};

#[post("/auth")]
pub async fn auth(login_data: Json<LoginRequest>, env: Data<Env>) -> Result<HttpResponse, HttpError> {
   let data = login_data.into_inner();
    
    let mut conn = env.pool.get()
        .map_err(|err|
            internal_error(format!("Database connection error: {}", err))
        )?;
    
    // Verify username
    let usr = block(move || {
        users
            .filter(name.eq(&data.name))
            .first::<User>(&mut conn)
    }).await
        .map_err(|_| 
            unauthorized("Invalid username or password")
        )?
        .map_err(|_|
            unauthorized("Invalid username or password")
        )?;
    
    // Verify password
    let password_matches = verify(&data.password, &usr.password)
        .map_err(|err| 
            internal_error(format!("Password verification error: {}", err))
        )?;
    
    if !password_matches {
        unauthorized("Invalid username or password");
    }
    
    // JWT token
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(env.jwt_expire))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: usr.id.to_string(),
        exp: expiration
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(env.jwt_secret.as_bytes()),
    ).map_err(|err| 
        internal_error(format!("Could not create a token: {}", err))
    )?;
    
    let json = AuthResponse {
        token: format!("Bearer {}", token)
    };
    
    Ok(HttpResponse::Ok().json(json))
}