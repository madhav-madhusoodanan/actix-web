use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
mod config;
// this function could be located in a different module

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config::config)
            .service(root)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
