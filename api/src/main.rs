use actix_web::{HttpServer, App, web::Data, middleware::Logger};
use actix_cors::Cors;

mod utils;
mod api;
mod db;
use crate::db::{connection, connection::DbPool};
use crate::api::users::auth::auth;
use crate::api::users::register::register;
use crate::api::event::new_event;

// Structure to hold environment configuration
#[derive(Clone)]
struct Env {
    pool: DbPool,
    jwt_secret: String,
    jwt_expire: i64,
}

impl Env {
    // Initialize the Env struct by reading environment variables
    async fn init() -> Self {
        let pool = connection::establish(get_env_var("DATABASE_URL")).await;

        Self {
            pool,
            jwt_secret: get_env_var("JWT_SECRET"),
            jwt_expire: get_env_var("JWT_EXPIRE").parse().unwrap_or(1)
        }
    }
}

// Helper function to read an environment variable
fn get_env_var(var: &str) -> String {
    std::env::var(var).unwrap_or_else(|_| panic!("Environment variable {} not set!", var))
}

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

    log::info!("Starting server on {}:{}", host, port);

    // Start the Actix web server
    HttpServer::new(move || {
        let logger = Logger::default();
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:8080")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::AUTHORIZATION,
            ])
            .supports_credentials();

        App::new()
            .wrap(logger)
            .wrap(cors)
            .app_data(Data::new(env_data.clone()))
            .service(register)
            .service(auth)
            .service(new_event)
            //.service(health)
    })
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
