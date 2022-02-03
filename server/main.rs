use actix_web::http::StatusCode;
use actix_web::*;
use std::io;

#[get("/health")]
async fn health() -> (String, StatusCode) {
    (String::from("healthy"), StatusCode::OK)
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let host = "0.0.0.0:8080";

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(health)
    })
    .bind(host)?
    .run()
    .await
}
