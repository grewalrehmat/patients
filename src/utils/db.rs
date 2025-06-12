use diesel::r2d2::{self, ConnectionManager, Pool};
use diesel::pg::PgConnection;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgPool {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in your .env file or environment");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}