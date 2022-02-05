use crate::{images, AppState};
use actix_web::*;
use image::{DynamicImage, Rgba};
use imageproc::drawing::{Canvas, draw_filled_rect_mut, draw_text_mut};
use imageproc::rect::Rect;
use rusttype::{Font, Scale};
use serde::Deserialize;
use shared::ImageError;
use conv::ValueInto;

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
) -> Result<HttpResponse> {
    let image = images::get_bytes(&data.client, &request.target_url).await;

    if let Err(e) = image {
        return Ok(HttpResponse::BadRequest().body(e.to_string()));
    }

    let image = image.unwrap();

    let font = Vec::from(CAPTION_FONT);
    let font = Font::try_from_vec(font).ok_or(ImageError::FontLoadFailure);

    if let Err(e) = font {
        return Ok(
            HttpResponse::BadRequest().body(e.to_string())
        );
    }

    let font = font.unwrap();
    let text = request.text.clone();


    let result = images::process(image, move |img| {
        let img = img.into_rgba8();
        let mut new_img = DynamicImage::new_rgba8(img.width(), img.height() + (img.width() / 5)).into_rgba8();
        let scale = (img.width() / 13) as f32;
        let scale = Scale { x: scale * 1.5, y: scale * 1.5 };

        let rect = Rect::at(0, 0).of_size(img.width(), img.width() / 5);
        let (size_x, size_y) = images::get_text_size(scale, &font, text.as_str());
        let size_x: u32 = size_x.value_into()?;
        let size_y: u32 = size_y.value_into()?;

        draw_filled_rect_mut(&mut new_img, rect, Rgba([255u8, 255u8, 255u8, 255u8]));
        draw_text_mut(
            &mut new_img,
            Rgba([0u8, 0u8, 0u8, 255u8]),
            img.width() / 2 - size_x / 2,
            img.width() / 10 - size_y / 2,
            scale,
            &font,
            text.as_str(),
        );

        let offset = img.width() / 5;

        for (x, y, pixel) in img.enumerate_pixels() {
            new_img.draw_pixel(x, y + offset, *pixel);
        }

        Ok(DynamicImage::ImageRgba8(new_img))
    })
    .await;

    if let Err(e) = result {
        return Ok(
            HttpResponse::BadRequest().body(format!("Failed to modify image. {}", e.to_string()))
        );
    }

    let (result, is_gif) = result.unwrap();

    Ok(HttpResponse::Ok()
        .append_header(http::header::ContentType(match is_gif {
            false => mime::IMAGE_PNG,
            true => mime::IMAGE_GIF
        }))
        .body(result))
}
