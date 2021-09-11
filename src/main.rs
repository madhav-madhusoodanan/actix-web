use actix_web::{get, App, HttpResponse, HttpServer, Responder};
mod routes;

// this function could be located in a different module

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(routes::config)
            .service(root)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
