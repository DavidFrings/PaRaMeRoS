use diesel_async::{
    AsyncPgConnection,
    pooled_connection::{AsyncDieselConnectionManager, bb8::Pool},
};

pub type DbPool = Pool<AsyncPgConnection>;

/// Build the database connection pool.
///
/// # Panics
/// If the pool cannot be created.
pub async fn establish(db_url: String) -> DbPool {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
    Pool::builder()
        .build(manager)
        .await
        .expect("Failed to create a pool!")
}
