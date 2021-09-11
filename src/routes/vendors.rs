use actix_web::{Scope, get, web,  HttpResponse, Responder};

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello this is vendor!")
}

#[get("/{id}")]
async fn vendor(params: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello this is vendor id:{} !", params.0))
}

// this function could be located in a different module
pub fn index() -> Scope{ 
    web::scope("/vendor")
           .service(root)
           .service(vendor)
}
