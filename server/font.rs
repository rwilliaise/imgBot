// use err_context::AnyError;
// use image::Rgba;
// use rusttype::Font;
//
// pub enum HorizontalGravity {
//     LeftGravity,
//     RightGravity,
//     CenterGravity,
// }
//
// pub enum VerticalGravity {
//     TopGravity,
//     BottomGravity,
//     CenterGravity,
// }
//
// #[derive(Clone)]
// pub struct DrawableFont {
//     inner: Font<'static>,
//     pixel: Rgba<u8>,
//     string: String,
// }
//
// impl DrawableFont {
//
//     pub fn text(&mut self, str: String) -> &mut Self {
//         self.string = str;
//         self
//     }
//
//     pub fn pixel(&mut self, pixel: <Rgba<u8> as Trait>::Pixel) -> &mut Self {
//         self.pixel = pixel;
//         self
//     }
//
//     pub fn flush(&mut self) -> Result<(), AnyError> {
//         Ok(())
//     }
// }
