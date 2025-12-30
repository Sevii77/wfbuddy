use crate::{Color, Image};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Theme {
	pub primary: Color,
	pub secondary: Color,
}

impl Theme {
	pub const WHITE: Self = Self{primary: Color::WHITE, secondary: Color::WHITE};
	
	/// Expects an image with a height of 1080
	pub fn from_options(image: Image) -> Self {
		const BAR_X: u32 = 110;
		const BAR_Y: u32 = 87;
		const BAR_W: u32 = 20;
		const BAR_H: u32 = 1;
		const MOUSE_X: u32 = 146;
		const MOUSE_Y: u32 = 181;
		const MOUSE_W: u32 = 14;
		const MOUSE_H: u32 = 8;
		
		Self {
			primary: image.sub_image(BAR_X, BAR_Y, BAR_W, BAR_H).average_color(),
			secondary: image.sub_image(MOUSE_X, MOUSE_Y, MOUSE_W, MOUSE_H).average_color(),
		}
	}
}