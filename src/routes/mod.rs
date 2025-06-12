use actix_web::web;
use crate::handlers::{auth, patients};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .service(auth::register_user)
        .service(auth::login_user)
        .configure(patients::configure);
}