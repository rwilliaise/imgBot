mod caption;

use actix_web::*;
use std::io;

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

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(health)
            .service(crate::caption::caption)
            .app_data(web::Data::new(AppState {
                client: reqwest::Client::builder()
                    .user_agent("imgBot-server")
                    .build()
                    .unwrap(),
            }))
    })
    .bind(host)?
    .run()
    .await
}
