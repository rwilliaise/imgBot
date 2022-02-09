use std::cmp::max;
use conv::ValueInto;
use err_context::AnyError;
use image::Rgba;
use imageproc::drawing::{Canvas, draw_text_mut};
use rusttype::{Font, Scale};
use textwrap::word_separators::{UnicodeBreakProperties, WordSeparator};
use crate::images;

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
pub struct DrawableFont {
    inner: Font<'static>,
    pixel: Rgba<u8>,
    string: String,
    scale: Scale,
    hor_gravity: HorizontalGravity,
    ver_gravity: VerticalGravity,
    width: u32,
    height: u32,
}

impl DrawableFont {
    pub fn new(font: Font) -> Self {
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

    pub fn color(&mut self, pixel: <Rgba<u8> as Trait>::Pixel) -> &mut Self {
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

    pub fn get_text_size(&self) -> (f32, f32) {
        let text = self.string.clone();
        let scale = self.scale.clone();
        let width = self.width.clone();

        let (w, _) = images::get_text_size(scale, &self.inner, "W");
        let cols = (width / w) - 1;

        let wrapped = textwrap::wrap(text.as_str(), cols);
        let metrics = self.inner.v_metrics(scale);

        let mut w = 0;
        let mut h = 0;
        for wrap in wrapped {
            let (text_width, _) = images::get_text_size(scale, &self.inner, &*wrap.clone());
            w = max(w, text_width.abs() as u32);
            h = h - metrics.descent + metrics.line_gap + metrics.ascent;
        }

        (w, h)
    }

    pub fn flush<C>(&mut self, img: &mut C, offset_x: i32, offset_y: i32) -> Result<(), AnyError>
    where
        C: Canvas<Pixel = Rgba<u8>>,
    {
        let text = std::mem::replace(&mut self.string, "".to_string());
        let color = std::mem::replace(&mut self.pixel, Rgba([0u8, 0u8, 0u8, 0u8]));
        let scale = std::mem::replace(&mut self.scale, Scale { x: 1., y: 1. });

        let width = std::mem::take(&mut self.width);
        let height = std::mem::take(&mut self.height);

        let (w) = images::get_text_size(scale, &self.inner, "W");
        let cols = (width / w) - 1;

        let wrapped = textwrap::wrap(text.as_str(), cols);
        let metrics = self.inner.v_metrics(scale);

        let mut wrap_y: f32 = 0.;
        for wrap in wrapped {
            let (text_width, text_height) = images::get_text_size(scale, &self.inner, &*wrap.clone());

            let offset_y = offset_y + wrap_y;

            wrap_y = wrap_y + metrics.descent - metrics.line_gap - metrics.ascent;

            let x = match std::mem::replace(&mut self.hor_gravity, HorizontalGravity::LeftGravity) {
                HorizontalGravity::LeftGravity => {
                    offset_x
                }
                HorizontalGravity::RightGravity => {
                    offset_x + width - text_width
                }
                HorizontalGravity::CenterGravity => {
                    offset_x + width / 2 - text_width / 2
                }
            };

            let y = match std::mem::replace(&mut self.ver_gravity, VerticalGravity::TopGravity) {
                VerticalGravity::TopGravity => {
                    offset_y
                }
                VerticalGravity::BottomGravity => {
                    offset_y + height - text_height
                }
                VerticalGravity::CenterGravity => {
                    offset_y + height / 2 - text_height / 2
                }
            };

            draw_text_mut(img, color, x.abs() as u32, y.abs() as u32, scale, &self.inner, text.as_str());
        }
        Ok(())
    }
}
