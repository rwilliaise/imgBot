use crate::font::{DrawableFont, HorizontalGravity, VerticalGravity};
use crate::images::ExploitableImageRequest;
use crate::{images};
use actix_web::*;
use bytes::Bytes;
use image::{DynamicImage, Rgba};
use rusttype::{Scale};
use shared::ImageError;

static SEVERED_FONT: &[u8] = include_bytes!("pack/severed.ttf");
static SEVERED_IMG: &[u8] = include_bytes!("pack/severed.png");

#[post("/severed")]
pub async fn severed(request: web::Json<ExploitableImageRequest>) -> Result<HttpResponse> {
    let image = Bytes::from(SEVERED_IMG);
    let font = DrawableFont::from(SEVERED_FONT);

    let text = request.text.clone();
    let scale = Scale { x: 102., y: 102. };

    let result = images::process(image, move |img| {
        let mut img = img.into_rgba8();

        let mut font = font.lock().unwrap();

        font.text(text.clone())
            .scale(scale)
            .color(Rgba([0u8, 255u8, 0u8, 255u8]))
            .extents(img.width() - 60, img.height() - 85)
            .gravity(
                HorizontalGravity::CenterGravity,
                VerticalGravity::TopGravity,
            )
            .flush(&mut img, 30., 85.).or(Err(ImageError::ProcessingFailure("Flush failure".to_string())))?;

        Ok(DynamicImage::ImageRgba8(img))
    })
    .await;

    if let Err(e) = result {
        return Ok(
            HttpResponse::BadRequest().body(format!("Failed to modify image. {}", e.to_string()))
        );
    }

    let (result, _) = result.unwrap();

    Ok(HttpResponse::Ok()
        .content_type(mime::IMAGE_PNG)
        .body(result))
}
