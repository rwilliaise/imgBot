
use actix_web::*;

#[post("/caption")]
pub async fn caption() -> Result<HttpResponse, Error> {
    println!("Received caption request.");
    Ok(HttpResponse::Ok().body("200 OK"))
}
