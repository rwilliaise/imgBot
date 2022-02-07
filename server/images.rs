use std::cmp::max;
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::env;
use std::collections::HashMap;

use actix_web::error::BlockingError;
use actix_web::web;
use bytes::Bytes;
use err_context::AnyError;
use image::codecs::gif;
use image::{AnimationDecoder, DynamicImage, Frame, ImageFormat};
use image::codecs::gif::Repeat;
use rusttype::{point, Font, Scale};

use shared::ImageError;

#[derive(serde::Deserialize)]
pub struct GenericImageRequest {
    pub target_url: String,
    pub text: String,
}

#[derive(serde::Deserialize)]
pub struct ExploitableImageRequest {
    pub text: String,
}

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
    f: impl Fn(DynamicImage) -> Result<DynamicImage, AnyError> + Send + Sync + Clone + 'static,
) -> Result<(Vec<u8>, bool), ImageError> {
    println!("Downloading...");

    let is_gif = Arc::new(AtomicBool::new(false));
    let copy = is_gif.clone();
    let get_frames: Result<Result<Vec<Frame>, AnyError>, BlockingError> = web::block(move || {
        let cursor = Cursor::new(bytes);
        let image = image::io::Reader::new(cursor.clone()).with_guessed_format()?;

        let format = &image
            .format()
            .ok_or(ImageError::BadImage("Not a valid format".to_string()))?;

        let frames: Vec<Frame>;
        match format {
            ImageFormat::Gif => {
                let decoder = gif::GifDecoder::new(cursor.clone())?;
                frames = decoder.into_frames().collect_frames()?;
                copy.store(true, Ordering::Relaxed);
            }
            _ => {
                let img = image.decode()?;
                let frame = Frame::new(img.to_rgba8());
                frames = vec![frame];
            }
        }

        Ok(frames)
    })
        .await;

    if let Err(e) = get_frames {
        return Err(ImageError::ProcessingFailure(e.to_string()));
    }

    let get_frames = get_frames.unwrap();

    if let Err(e) = get_frames {
        return Err(ImageError::ProcessingFailure(e.to_string()));
    }

    println!("Executing job...");

    let frames = get_frames.unwrap();

    let new_frames = Arc::new(Mutex::new(HashMap::with_capacity(frames.len())));
    let mut joinables = Vec::new();
    for (i, frame) in frames.iter().enumerate() {
        let new_frames = new_frames.clone();
        let frame = frame.clone();
        let f = f.clone();
        joinables.push(web::block(move || {
            let mut new_frames = new_frames.lock().unwrap();
            let image = frame.buffer();
            let image = f(DynamicImage::ImageRgba8(image.clone()))
                .unwrap()
                .to_rgba8();

            new_frames.insert(i, Frame::from_parts(
                image,
                frame.left(),
                frame.top(),
                frame.delay()
            ));
        }));
    }

    futures::future::join_all(joinables).await;

    let is_gif = is_gif.load(Ordering::Relaxed);

    println!("Start encoding...");

    let try_image: std::result::Result<
        std::result::Result<(Vec<u8>, bool), AnyError>,
        BlockingError,
    > = web::block(move || {
        let new_frames = new_frames.lock().unwrap();

        let tmp = match env::var("KUBERNETES_SERVICE_HOST") {
            Ok(_) => {
                // we are running in k8s
                "/tmp/"
            }
            Err(_) => "./tmp",
        };

        let buf = PathBuf::from(format!("{}{}", tmp, uuid::Uuid::new_v4().to_string()));

        if is_gif {
            let mut out = Vec::new();
            let mut encoder = gif::GifEncoder::new_with_speed(&mut out, 20);
            for (_, frame) in new_frames.iter().sorted_by_key(|k| k.0) {
                encoder.encode_frame(frame.clone())?;
            }
            encoder.set_repeat(Repeat::Infinite)?;

            drop(encoder);

            return Ok((out, true));
        } else if new_frames.len() > 0 {
            if new_frames.len() > 1 {
                println!("Residual frames detected");
            }
            let image = new_frames.get(&0).unwrap();
            let image = DynamicImage::ImageRgba8(image.buffer().clone());
            match image {
                DynamicImage::ImageRgba8(e) => e.save_with_format(&buf, ImageFormat::Png)?,
                _ => {
                    return Err(ImageError::BadRequest("Unsupported format!".to_string()).into())
                }
            }

            let bytes = std::fs::read(&buf)?;

            std::fs::remove_file(&buf)?;

            return Ok((bytes, false));
        }

        Err(ImageError::ProcessingFailure("No frames created".to_string()).into())
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
