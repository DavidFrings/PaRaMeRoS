pub mod api;
pub mod app;
pub mod client_ip;
pub mod db;
pub mod media;
pub mod migrate;
pub mod redis_store;
pub mod utils;

use crate::db::{connection, connection::DbPool};
use std::path::PathBuf;

/// Struct to hold environment configuration
#[derive(Clone)]
pub struct Env {
    pub pool: DbPool,
    pub jwt_secret: String,
    pub jwt_expire: i64,
    pub admin_pass: String,
    pub cors_origins: Vec<String>,
    pub upload_dir: PathBuf,
    pub redis: Option<redis::aio::ConnectionManager>,
    pub site_url: String,
}

/// Leaves out the secrets and the pool so `{:?}` cannot leak them into logs.
impl std::fmt::Debug for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Env")
            .field("jwt_expire", &self.jwt_expire)
            .field("cors_origins", &self.cors_origins)
            .field("upload_dir", &self.upload_dir)
            .field("redis", &self.redis.is_some())
            .field("site_url", &self.site_url)
            .finish_non_exhaustive()
    }
}

impl Env {
    /// Initialize the Env struct by reading environment variables
    pub async fn init() -> Self {
        let pool = connection::establish(get_env_var("DATABASE_URL")).await;

        let jwt_secret = get_env_var("JWT_SECRET");
        let admin_pass = get_env_var("ADMIN_PASS");
        if jwt_secret.len() < 32 {
            log::warn!("JWT_SECRET is shorter than 32 bytes; use a long random secret");
        }
        if admin_pass.len() < 12 {
            log::warn!("ADMIN_PASS is shorter than 12 characters; use a long random password");
        }

        Self {
            pool,
            jwt_secret,
            jwt_expire: get_env_var("JWT_EXPIRE").parse().unwrap_or(1),
            admin_pass,
            cors_origins: get_env_var_optional("CORS_ORIGINS")
                .map(|value| {
                    value
                        .split(',')
                        .map(str::trim)
                        .filter(|origin| !origin.is_empty())
                        .map(str::to_owned)
                        .collect::<Vec<String>>()
                })
                .filter(|origins| !origins.is_empty())
                .unwrap_or_else(default_cors_origins),
            upload_dir: get_env_var_optional("UPLOAD_DIR")
                .map_or_else(|| PathBuf::from("/api/uploads"), PathBuf::from),
            redis: connect_redis().await,
            site_url: get_env_var("SITE_URL"),
        }
    }
}

/// Helper function to read an environment variable
///
/// # Panics
/// If `var` is not set.
#[must_use]
pub fn get_env_var(var: &str) -> String {
    std::env::var(var).unwrap_or_else(|_| panic!("Environment variable {var} not set!"))
}

#[must_use]
pub fn get_env_var_optional(var: &str) -> Option<String> {
    std::env::var(var).ok()
}

fn default_cors_origins() -> Vec<String> {
    vec![
        "http://localhost:3000".to_string(),
        "https://www.PaRaMeRoS.net".to_string(),
        "https://PaRaMeRoS.net".to_string(),
        "https://PaRaMeRoS.DavidFrings.dev".to_string(),
    ]
}

/// Connect to `REDIS_URL` with a 2 s timeout; `None` (rate limiting/caching off) on failure.
async fn connect_redis() -> Option<redis::aio::ConnectionManager> {
    let url = get_env_var_optional("REDIS_URL")?;
    let connect = async { redis::aio::ConnectionManager::new(redis::Client::open(url)?).await };
    match actix_web::rt::time::timeout(std::time::Duration::from_secs(2), connect).await {
        Ok(Ok(manager)) => Some(manager),
        Ok(Err(err)) => {
            log::warn!("Redis unavailable, continuing without it: {err}");
            None
        }
        Err(_) => {
            log::warn!("Redis connection timed out, continuing without it");
            None
        }
    }
}
