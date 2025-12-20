mod image;
pub use image::*;

mod ocr;
pub use ocr::OCR;

pub(crate) mod theme;
pub mod screen;

pub struct Theme {
	pub primary: Color,
	pub secondary: Color,
}