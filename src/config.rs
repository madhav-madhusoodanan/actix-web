use actix_web::{get, web, HttpResponse, Responder};

// this function could be located in a different module
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(app);
}

#[get("/app")]
async fn app() -> impl Responder {
    HttpResponse::Ok().body("Hello this is app!")
}