use crate::AppState;
use actix_web::*;
use err_context::AnyError;
use image::DynamicImage;
use serde::Deserialize;
use std::io::Cursor;

const CAPTION_FONT: &[u8] = include_bytes!("pack/caption.otf");

#[derive(Deserialize)]
pub struct CaptionRequest {
    pub target_url: String,
    pub text: String,
}

#[post("/caption")]
pub async fn caption(
    request: web::Json<CaptionRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let response = data.client.get(&request.target_url).send().await;

    if let Err(e) = response {
        dbg!(e);
        return Ok(HttpResponse::BadRequest().body("Bad request"));
    }

    let response = response.unwrap().bytes().await;

    if let Err(e) = response {
        dbg!(e);
        return Ok(HttpResponse::BadRequest().body("Bad image"));
    }

    let try_image: std::result::Result<DynamicImage, AnyError> = (|| {
        let mut image = image::io::Reader::new(Cursor::new(response.unwrap()))
            .with_guessed_format()?
            .decode()?;

        Ok(image)
    })();

    Ok(HttpResponse::Ok()
        .append_header(http::header::ContentType(mime::IMAGE_PNG))
        .body(response.unwrap()))
}
