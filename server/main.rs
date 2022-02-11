use std::io;

use actix_web::*;

mod caption;
mod font;
mod images;
mod severed;

pub struct AppState {
    client: reqwest::Client,
}

#[get("/health")]
async fn health() -> Result<HttpResponse, error::Error> {
    Ok(HttpResponse::Ok().body("200 OK"))
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let host = "0.0.0.0:8080";

    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(health)
            .service(crate::caption::caption)
            .service(crate::severed::severed)
            .app_data(web::Data::new(AppState {
                client: reqwest::Client::builder()
                    .user_agent("imgBot-server")
                    .build()
                    .unwrap(),
            }))
    })
    .bind(host)?;

    println!("Starting server!");
    server.run().await
}
