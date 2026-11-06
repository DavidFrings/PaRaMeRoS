pub mod api;
pub mod app;
pub mod db;
pub mod media;
pub mod migrate;
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

impl Env {
    /// Initialize the Env struct by reading environment variables
    pub async fn init() -> Self {
        let pool = connection::establish(get_env_var("DATABASE_URL")).await;

        Self {
            pool,
            jwt_secret: get_env_var("JWT_SECRET"),
            jwt_expire: get_env_var("JWT_EXPIRE").parse().unwrap_or(1),
            admin_pass: get_env_var("ADMIN_PASS"),
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
            redis: None,
            site_url: get_env_var_optional("SITE_URL")
                .unwrap_or_else(|| "http://localhost:3000".to_string()),
        }
    }
}

/// Helper function to read an environment variable
pub fn get_env_var(var: &str) -> String {
    std::env::var(var).unwrap_or_else(|_| panic!("Environment variable {} not set!", var))
}

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
