use actix_web::{App, HttpServer, middleware::Logger, web::Data};
use log::{error, info};
use parameros_api::{Env, app, get_env_var, media::backfill::backfill, migrate::run_migrations};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Only runs if not in release mode
    #[cfg(debug_assertions)]
    {
        dotenvy::dotenv().ok();
        // Rust 2024 marks set_var unsafe; there is no safe way to set these before startup.
        #[allow(unsafe_code)]
        // SAFETY: runs right after dotenv, before the logger, pool or any thread, so no other
        // thread can read the environment concurrently.
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

    info!("Starting server on {host}:{port}");

    let upload_dir = env_data.upload_dir.clone();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(app::cors(&env_data.cors_origins))
            .app_data(Data::new(env_data.clone()))
            .configure(app::configure)
    })
    .bind(format!("{host}:{port}"))?
    .run();

    actix_web::rt::spawn(backfill(upload_dir));
    server.await
}
