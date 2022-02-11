// TODO: pango?

use crate::images;
use err_context::AnyError;
use image::Rgba;
use imageproc::drawing::{draw_text_mut, Canvas};
use rusttype::{Font, Scale};
use std::sync::{Arc, Mutex};
use textwrap::Options;

static TEST_GLYPH: char = 'o';

#[derive(Clone)]
pub enum HorizontalGravity {
    LeftGravity,
    RightGravity,
    CenterGravity,
}

#[derive(Clone)]
pub enum VerticalGravity {
    TopGravity,
    BottomGravity,
    CenterGravity,
}

#[derive(Clone)]
pub struct DrawableFont<'a> {
    inner: Font<'a>,
    pixel: Rgba<u8>,
    string: String,
    scale: Scale,
    hor_gravity: HorizontalGravity,
    ver_gravity: VerticalGravity,
    width: u32,
    height: u32,
}

impl<'a> DrawableFont<'a> {
    pub fn from(data: &'static [u8]) -> Arc<Mutex<Self>> {
        let font = Vec::from(data);
        let font = Font::try_from_vec(font).unwrap();
        Arc::new(Mutex::new(DrawableFont::new(font)))
    }

    pub fn new(font: Font<'a>) -> Self {
        Self {
            inner: font.clone(),
            string: "".to_string(),
            pixel: Rgba([0u8, 0u8, 0u8, 0u8]),
            scale: Scale { x: 1., y: 1. },
            hor_gravity: HorizontalGravity::LeftGravity,
            ver_gravity: VerticalGravity::TopGravity,
            width: 0,
            height: 0,
        }
    }

    pub fn text(&mut self, str: String) -> &mut Self {
        self.string = str;
        self
    }

    pub fn scale(&mut self, scale: Scale) -> &mut Self {
        self.scale = scale;
        self
    }

    pub fn color(&mut self, pixel: Rgba<u8>) -> &mut Self {
        self.pixel = pixel;
        self
    }

    pub fn extents(&mut self, width: u32, height: u32) -> &mut Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn gravity(&mut self, hor: HorizontalGravity, ver: VerticalGravity) -> &mut Self {
        self.hor_gravity = hor;
        self.ver_gravity = ver;
        self
    }

    pub fn get_text_size(&self) -> (u32, u32) {
        let text = self.string.clone();
        let scale = self.scale.clone();
        let width: f32 = self.width.clone() as f32;

        let (w, _) = images::get_glyph_size(scale, &self.inner, TEST_GLYPH);
        let cols: f32 = width / w as f32 - 1.;

        let wrapped = textwrap::wrap(text.as_str(), cols as usize);
        let metrics = self.inner.v_metrics(scale);

        let mut w: f32 = 0.;
        let mut h: f32 = 0.;
        for wrap in wrapped {
            let wrap = wrap.to_string();
            let wrap = wrap.trim();
            let (text_width, _) = images::get_text_size(scale, &self.inner, &*wrap.clone());
            w = w.max(text_width.abs() as f32);
            h = h - metrics.descent + metrics.line_gap + metrics.ascent;
        }

        (w as u32, h as u32)
    }

    pub fn flush<C>(&mut self, img: &mut C, offset_x: f32, offset_y: f32) -> Result<(), AnyError>
    where
        C: Canvas<Pixel = Rgba<u8>>,
    {
        let (_, total_height) = self.get_text_size();
        let total_height = total_height as f32;

        let text = std::mem::replace(&mut self.string, "".to_string());
        let color = std::mem::replace(&mut self.pixel, Rgba([0u8, 0u8, 0u8, 0u8]));
        let scale = std::mem::replace(&mut self.scale, Scale { x: 1., y: 1. });

        let hor_gravity = std::mem::replace(&mut self.hor_gravity, HorizontalGravity::LeftGravity);
        let ver_gravity = std::mem::replace(&mut self.ver_gravity, VerticalGravity::TopGravity);

        let width = std::mem::take(&mut self.width) as f32;
        let height = std::mem::take(&mut self.height) as f32;

        let (w, _) = images::get_glyph_size(scale, &self.inner, TEST_GLYPH);
        dbg!(w);
        let w = w as f32;
        let cols = (width / w) - 1.;

        let wrapped = textwrap::wrap(
            text.as_str(),
            Options::new(cols as usize).wrap_algorithm(textwrap::wrap_algorithms::OptimalFit),
        );
        let metrics = self.inner.v_metrics(scale);

        let mut wrap_y: f32 = 0.;
        for wrap in wrapped {
            let wrap = wrap.to_string();
            let wrap = wrap.trim();
            let (text_width, _) = images::get_text_size(scale, &self.inner, &*wrap.clone());
            let text_width = text_width as f32;
            let offset_y = offset_y + wrap_y;

            wrap_y = wrap_y - metrics.descent + metrics.line_gap + metrics.ascent;

            let x = match hor_gravity {
                HorizontalGravity::LeftGravity => offset_x,
                HorizontalGravity::RightGravity => offset_x + width - text_width,
                HorizontalGravity::CenterGravity => offset_x + width / 2. - text_width / 2.,
            };

            let y = match ver_gravity {
                VerticalGravity::TopGravity => offset_y,
                VerticalGravity::BottomGravity => offset_y + height - total_height,
                VerticalGravity::CenterGravity => offset_y + height / 2. - total_height / 2.,
            };

            draw_text_mut(
                img,
                color,
                x.abs() as u32,
                y.abs() as u32,
                scale,
                &self.inner,
                &*wrap.clone(),
            );
        }
        Ok(())
    }
}
