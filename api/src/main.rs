mod api;
mod db;
mod utils;

use crate::api::post::delete_post::delete_post;
use crate::api::post::{new_post::new_post, post::post, posts::posts, update_post::update_post};
use crate::api::users::{auth::auth, register::register, verify::auth_verify};
use crate::db::{connection, connection::DbPool};
use actix_cors::Cors;
use actix_web::{
    App, HttpServer,
    http::header::{AUTHORIZATION, CONTENT_TYPE},
    middleware::Logger,
    web::Data,
};
use log::info;

const UPLOAD_DIR: &str = "/api/uploads";

/// Struct to hold environment configuration
#[derive(Clone)]
pub(crate) struct Env {
    pool: DbPool,
    jwt_secret: String,
    jwt_expire: i64,
    admin_pass: String,
}

impl Env {
    /// Initialize the Env struct by reading environment variables
    async fn init() -> Self {
        let pool = connection::establish(get_env_var("DATABASE_URL")).await;

        Self {
            pool,
            jwt_secret: get_env_var("JWT_SECRET"),
            jwt_expire: get_env_var("JWT_EXPIRE").parse().unwrap_or(1),
            admin_pass: get_env_var("ADMIN_PASS"),
        }
    }
}

/// Helper function to read an environment variable
fn get_env_var(var: &str) -> String {
    std::env::var(var).unwrap_or_else(|_| panic!("Environment variable {} not set!", var))
}

/// Health check endpoint
//#[get("/health")]
//async fn health() -> HttpResponse {
//    HttpResponse::Ok().body("Ok")
//}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Only runs if not in release mode
    #[cfg(debug_assertions)]
    {
        dotenv::dotenv().ok();
        unsafe {
            std::env::set_var("RUST_LOG", "debug");
            std::env::set_var("RUST_BACKTRACE", "1");
        }
    }

    env_logger::init();

    let host = get_env_var("HOST");
    let port = get_env_var("PORT");
    let env_data = Env::init().await;

    info!("Starting server on {}:{}", host, port);

    // Start the Actix web server
    HttpServer::new(move || {
        let logger = Logger::default();
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("https://www.PaRaMeRoS.net")
            .allowed_origin("https://PaRaMeRoS.net")
            .allowed_origin("https://PaRaMeRoS.DavidFrings.dev")
            .allowed_origin_fn(|origin, _req_head| {
                // Postman & Curl
                if origin.as_bytes().is_empty() {
                    true
                } else {
                    false
                }
            })
            //.allow_any_origin() // Dev
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![CONTENT_TYPE, AUTHORIZATION])
            //.allow_any_header() // Dev
            .supports_credentials();

        App::new()
            .wrap(logger)
            .wrap(cors)
            .app_data(Data::new(env_data.clone()))
            .service(register)
            .service(auth)
            .service(auth_verify)
            .service(post)
            .service(posts)
            .service(new_post)
            .service(update_post)
            .service(delete_post)
          //.service(health)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
