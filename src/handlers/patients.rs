use actix_web::{get, web, HttpResponse, Responder};

#[get("/patients")]
async fn get_patients() -> impl Responder {
    HttpResponse::Ok().body("All patients would be listed here.")
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_patients);
}