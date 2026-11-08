use diesel_async::{
    AsyncConnection, AsyncPgConnection, async_connection_wrapper::AsyncConnectionWrapper,
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

/// Apply all pending embedded migrations (sync wrapper over the async connection, no libpq).
///
/// # Errors
/// If connecting to the database or a migration fails.
pub async fn run_migrations(database_url: &str) -> anyhow::Result<()> {
    let conn = AsyncPgConnection::establish(database_url).await?;
    actix_web::rt::task::spawn_blocking(move || {
        let mut conn: AsyncConnectionWrapper<AsyncPgConnection> = conn.into();
        conn.run_pending_migrations(MIGRATIONS)
            .map(|_| ())
            .map_err(|err| anyhow::anyhow!("{err}"))
    })
    .await?
}
