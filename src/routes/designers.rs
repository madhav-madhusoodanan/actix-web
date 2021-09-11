use actix_web::{Scope, get, web,  HttpResponse, Responder};

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello this is designers!")
}

#[get("/{id}")]
async fn designer(params: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello this is designer id:{} !", params.0))
}

// this function could be located in a different module
pub fn index() -> Scope{ 
    web::scope("/designers")
           .service(root)
           .service(designer)
}
