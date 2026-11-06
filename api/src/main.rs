use actix_web::{App, HttpServer, middleware::Logger, web::Data};
use log::{error, info};
use parameros_api::{Env, app, get_env_var, migrate::run_migrations};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Only runs if not in release mode
    #[cfg(debug_assertions)]
    {
        dotenvy::dotenv().ok();
        unsafe {
            std::env::set_var("RUST_LOG", "debug");
            std::env::set_var("RUST_BACKTRACE", "1");
        }
    }

    env_logger::init();

    let host = get_env_var("HOST");
    let port = get_env_var("PORT");

    if let Err(err) = run_migrations(&get_env_var("DATABASE_URL")).await {
        error!("Migrations failed: {err}");
        std::process::exit(1);
    }

    let env_data = Env::init().await;

    info!("Starting server on {}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(app::cors(&env_data.cors_origins))
            .app_data(Data::new(env_data.clone()))
            .configure(app::configure)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
