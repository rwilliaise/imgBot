use actix_web::error::BlockingError;
use actix_web::web;
use bytes::Bytes;
use err_context::AnyError;
use image::codecs::gif;
use image::{AnimationDecoder, DynamicImage, Frame, Frames, ImageFormat};
use rusttype::{point, Font, Scale};
use shared::ImageError;
use std::cmp::max;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Cursor, Write};
use std::path::PathBuf;
use image::codecs::gif::Repeat;

pub async fn get_bytes(client: &reqwest::Client, target_url: &String) -> Result<Bytes, ImageError> {
    let response = client.get(target_url).send().await;

    if let Err(e) = response {
        return Err(ImageError::BadRequest(e.to_string()));
    }

    let response = response.unwrap().bytes().await;

    if let Err(e) = response {
        return Err(ImageError::BadImage(e.to_string()));
    }

    Ok(response.unwrap())
}

pub fn get_text_size(scale: Scale, font: &Font, text: &str) -> (i32, i32) {
    let v_metrics = font.v_metrics(scale);

    let (mut w, mut h) = (0, 0);

    for g in font.layout(text, scale, point(0.0, v_metrics.ascent)) {
        if let Some(bb) = g.pixel_bounding_box() {
            w = max(w, bb.max.x);
            h = max(h, bb.max.y);
        }
    }

    (w, h)
}

pub async fn process(
    bytes: Bytes,
    f: impl Fn(DynamicImage) -> Result<DynamicImage, AnyError> + Send + Sync + 'static,
) -> Result<(Vec<u8>, bool), ImageError> {
    let try_image: std::result::Result<std::result::Result<(Vec<u8>, bool), AnyError>, BlockingError> =
        web::block(move || {
            let cursor = Cursor::new(bytes);
            let image = image::io::Reader::new(cursor.clone()).with_guessed_format()?;

            let format = &image.format().ok_or(ImageError::BadImage(
                "Not a valid format".to_string(),
            ))?;

            let mut frames: Vec<Frame>;
            let mut is_gif = false;
            match format {
                ImageFormat::Gif => {
                    let decoder = gif::GifDecoder::new(cursor.clone())?;
                    frames = decoder.into_frames().collect_frames()?;
                    is_gif = true;
                }
                _ => {
                    let img = image.decode()?;
                    let frame = Frame::new(img.to_rgba8());
                    frames = vec![frame];
                }
            }

            let mut new_frames = Vec::new();
            for frame in frames {
                let image = frame.buffer();
                let image = f(DynamicImage::ImageRgba8(image.clone()))?.to_rgba8();
                new_frames.push(Frame::from_parts(
                    image,
                    frame.top(),
                    frame.left(),
                    frame.delay(),
                ));
            }

            let tmp = match env::var("KUBERNETES_SERVICE_HOST") {
                Ok(_) => {
                    // we are running in k8s
                    "/tmp/"
                }
                Err(_) => "./tmp",
            };

            let buf = PathBuf::from(format!("{}{}", tmp, uuid::Uuid::new_v4().to_string()));

            if is_gif {
                let fout = &mut BufWriter::new(File::create(&buf)?);
                let mut encoder = gif::GifEncoder::new(fout);
                for frame in new_frames {
                    encoder.encode_frame(frame)?;
                }
                encoder.set_repeat(Repeat::Infinite)?;
            } else if new_frames.len() > 0 {
                if new_frames.len() > 1 {
                    println!("Residual frames detected");
                }
                let image = new_frames.get(0).unwrap();
                let image = DynamicImage::ImageRgba8(image.buffer().clone());
                match image {
                    DynamicImage::ImageRgba8(e) => e.save_with_format(&buf, ImageFormat::Png)?,
                    _ => {
                        return Err(ImageError::BadRequest("Unsupported format!".to_string()).into())
                    }
                }
            }

            let bytes = std::fs::read(&buf)?;

            std::fs::remove_file(&buf)?;

            Ok((bytes, is_gif))
        })
        .await;

    if let Err(e) = try_image {
        return Err(ImageError::ProcessingFailure(e.to_string()));
    }

    let try_image = try_image.unwrap();

    if let Err(e) = try_image {
        return Err(ImageError::ProcessingFailure(e.to_string()));
    }

    Ok(try_image.unwrap())
}
