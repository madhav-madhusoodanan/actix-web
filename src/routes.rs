use actix_web::{get, web, HttpResponse, Responder};

pub mod paintings;

// this function could be located in a different module
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(paintings::index)
        .service(app);
}

#[get("/config")]
async fn app() -> impl Responder {
    HttpResponse::Ok().body("Hello this is config!")
}