use actix_web::{Scope, get, web,  HttpResponse, Responder};

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello this is paintings!")
}

#[get("/{id}")]
async fn painting(params: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello this is paintings id:{} !", params.0))
}

// this function could be located in a different module
pub fn index() -> Scope{ 
    web::scope("/paintings")
           .service(root)
           .service(painting)
}
