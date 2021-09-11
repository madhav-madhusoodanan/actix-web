use actix_web::{App, HttpServer};
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /* Factory is just a router */
    let factory = || App::new().configure(routes::config);

    /* Initiating the server */
    HttpServer::new(factory)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
