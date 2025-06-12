mod models;
mod schema;
mod routes;
mod handlers;
mod utils;
mod middleware;

use actix_web::{App, HttpServer};
use actix_cors::Cors;
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use middleware::auth::AuthMiddleware;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
use dotenvy::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(AuthMiddleware) // 🔒 protect routes
            .app_data(actix_web::web::Data::new(pool.clone()))
            .configure(routes::configure)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}