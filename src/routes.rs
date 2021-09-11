use actix_web::{get, web, HttpResponse, Responder};

pub mod paintings;
pub mod designers;
pub mod vendors;

// this function could be located in a different module
pub fn config(config: &mut web::ServiceConfig) {
    config
        .service(paintings::index())
        .service(designers::index())
        .service(vendors::index())
        .service(app)
        .service(root);
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/config")]
async fn app() -> impl Responder {
    HttpResponse::Ok().body("Hello this is config!")
}