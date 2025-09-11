use diesel_async::{
    AsyncPgConnection,
    pooled_connection::{AsyncDieselConnectionManager, bb8::Pool},
};

pub type DbPool = Pool<AsyncPgConnection>;

pub async fn establish(db_url: String) -> DbPool {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
    Pool::builder()
        .build(manager)
        .await
        .expect("Failed to create a pool!")
}
