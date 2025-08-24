use diesel::{pg::PgConnection, r2d2::{self, ConnectionManager}};
use r2d2::Pool;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish(db_url: String) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create a pool!")
}