use actix_web::{get, HttpResponse, Responder};

#[get("/paintings")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello this is paintings!")
}
