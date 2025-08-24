use actix_web::{HttpServer, App, web::Data, middleware::Logger};
use dotenv::dotenv;
use crate::api::event::new_event;

mod utils;
mod api;
mod db;
use crate::db::{connection, connection::DbPool};
use crate::api::users::auth::auth;
use crate::api::users::register::register;

// Structure to hold environment configuration
#[derive(Clone)]
struct Env {
    pool: DbPool,
    jwt_secret: String,
    jwt_expire: i64,
}

impl Env {
    // Initialize the Env struct by reading environment variables
    fn init() -> Self {
        let pool = connection::establish(get_env_var("DATABASE_URL"));
        
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Only runs if not in release mode
    #[cfg(debug_assertions)]
    {
        dotenv().ok();
        unsafe {
            std::env::set_var("RUST_LOG", "debug");
            std::env::set_var("RUST_BACKTRACE", "1");
        }
    }

    env_logger::init();

    let host = get_env_var("HOST");
    let port = get_env_var("PORT");
    let env_data = Env::init();

    log::info!("Starting server on {}:{}", host, port);

    // Start the Actix web server
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(Data::new(env_data.clone()))
            .service(register)
            .service(auth)
            .service(new_event)
    })
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
