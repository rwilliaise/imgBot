use std::borrow::Borrow;
use std::env;
use crate::AppState;
use actix_web::*;
use err_context::AnyError;
use image::{DynamicImage, ImageFormat, Rgb, Rgba};
use serde::Deserialize;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use shared::CommandError;

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
        return Ok(HttpResponse::BadRequest().body(format!("Bad request. {}", e.to_string())));
    }

    let response = response.unwrap().bytes().await;

    if let Err(e) = response {
        return Ok(HttpResponse::BadRequest().body(format!("Bad image. {}", e.to_string())));
    }

    let try_image: std::result::Result<Vec<u8>, AnyError> = (|| {
        let image = image::io::Reader::new(Cursor::new(response.unwrap()))
            .with_guessed_format()?
            .decode()?;

        let font = Vec::from(CAPTION_FONT);
        let font = Font::try_from_vec(font).ok_or(CommandError::GenericError("Font load fail"))?;

        let scale = 24.8;
        let scale = Scale {
            x: scale * 2.0,
            y: scale
        };

        let text = request.text.as_str();
        let tmp =  match env::var("KUBERNETES_SERVICE_HOST") {
            Ok(_) => {
                // we are running in k8s
                "/tmp/"
            }
            Err(_) => "./tmp",
        };

        let buf = PathBuf::from(format!("{}{}", tmp, uuid::Uuid::new_v4().to_string()));

        match image {
            DynamicImage::ImageRgb8(mut e) => {
                draw_text_mut(&mut e, Rgb([255u8, 255u8, 255u8]), 10, 10, scale, &font, text);
                e.save_with_format(&buf, ImageFormat::Png)?;
            },
            DynamicImage::ImageRgba8(mut e) => {
                draw_text_mut(&mut e, Rgba([255u8, 255u8, 255u8, 255u8]), 10, 10, scale, &font, text);
                e.save_with_format(&buf, ImageFormat::Png)?;
            },
            _ => {
                return Err(CommandError::GenericError("Unsupported format.").into());
            }
        };

        Ok(std::fs::read(&buf)?)
    })();

    if let Err(e) = try_image {
        return Ok(HttpResponse::BadRequest().body(format!("Failed to load image. {}", e.to_string())));
    }



    Ok(HttpResponse::Ok()
        .append_header(http::header::ContentType(mime::IMAGE_PNG))
        .body(try_image.unwrap()))
}
