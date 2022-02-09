use actix_web::*;
use conv::ValueInto;
use image::{DynamicImage, Rgba};
use imageproc::drawing::{Canvas, draw_filled_rect_mut, draw_text_mut};
use imageproc::rect::Rect;
use rusttype::{Font, Scale};

use crate::{AppState, images};
use crate::font::{DrawableFont, HorizontalGravity, VerticalGravity};
use crate::images::GenericImageRequest;

const CAPTION_FONT: &[u8] = include_bytes!("pack/caption.otf");


#[post("/caption")]
pub async fn caption(
    request: web::Json<GenericImageRequest>,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    let image = images::get_bytes(&data.client, &request.target_url).await;

    if let Err(e) = image {
        return Ok(HttpResponse::BadRequest().body(e.to_string()));
    }

    let image = image.unwrap();

    let font = Vec::from(CAPTION_FONT);
    let font = Font::try_from_vec(font).unwrap();

    let mut font = DrawableFont::new(font);

    let text = request.text.clone();

    let result = images::process(image, move |img| {
        let img = img.into_rgba8();

        let scale = (img.width() / 13) as f32;
        let scale = Scale { x: scale * 1.5, y: scale * 1.5 };

        font.text(text)
            .scale(scale)
            .color(Rgba([0u8, 0u8, 0u8, 255u8]))
            .extents(img.width(), img.height())
            .gravity(HorizontalGravity::CenterGravity, VerticalGravity::TopGravity);

        let (_, h) = font.get_text_size();

        let offset: f32 = h + img.width() / 13;
        let mut new_img = DynamicImage::new_rgba8(img.width(), img.height() + offset).into_rgba8();
        let rect = Rect::at(0, 0).of_size(img.width(), offset as u32);
        font.extents(img.width(), offset as u32);

        draw_filled_rect_mut(&mut new_img, rect, Rgba([255u8, 255u8, 255u8, 255u8]));
        font.flush(&mut new_img, 0, 0);

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
