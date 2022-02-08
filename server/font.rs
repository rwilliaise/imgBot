use err_context::AnyError;
use image::Rgba;
use imageproc::drawing::Canvas;
use rusttype::{Font, Scale};

pub enum HorizontalGravity {
    LeftGravity,
    RightGravity,
    CenterGravity,
}

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
    scale: Scale
}

impl DrawableFont {
    pub fn new(font: &Font) -> Self {
        Self {
            inner: font.clone(),
            string: "".to_string(),
            pixel: Rgba([0u8, 0u8, 0u8, 0u8]),
            scale: Scale { x: 1., y: 1. }
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

    pub fn extents(&mut self, (width, height): (i32, i32)) -> &mut Self {

        self
    }

    pub fn gravity(&mut self, hor: HorizontalGravity, ver: VerticalGravity) -> &mut Self {

        self
    }

    pub fn flush<C>(&mut self, img: &mut C) -> Result<(), AnyError>
    where
        C: Canvas<Pixel = Rgba<u8>>,
    {
        let cols = (img.width() / (w.abs() as u32)) - 1;
        let text = std::mem::replace(&mut self.string, "".to_string());
        let color = std::mem::replace(&mut self.pixel, Rgba([0u8, 0u8, 0u8, 0u8]));
        let scale = std::mem::replace(&mut self.scale, Scale { x: 1., y: 1. });

        let x =
        Ok(())
    }
}
