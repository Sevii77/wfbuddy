use crate::{Image, Theme};

pub fn from_party_header_text(image: Image, offset_secondary: u32) -> Theme {
	const AVATAR_START: u32 = 96;
	const AVATAR_SIZE: u32 = 44;
	const AVATAR_SPACING: u32 = 4;
	const AVATAR_BAR_Y: u32 = 85;
	const AVATAR_BAR_W: u32 = 40;
	const AVATAR_BAR_H: u32 = 3;
	const TEXT_Y: u32 = 49;
	const TEXT_CHECK_WIDTH: u32 = 2;
	const TEXT_CHECK_HEIGHT: u32 = 36;
	
	let primary_color = image.sub_image(AVATAR_START + 1, AVATAR_BAR_Y, AVATAR_BAR_W, AVATAR_BAR_H).average_color();
	println!("primary color is {primary_color:?}");
	
	// not actually playercount, if less than 4 it'll count the +, but that works for our purpose
	let mut player_count = 1;
	for i in 1..4 {
		let bar_color = image.sub_image(AVATAR_START + (AVATAR_SIZE + AVATAR_SPACING) * i + 1, AVATAR_BAR_Y, AVATAR_BAR_W, AVATAR_BAR_H).average_color();
		let deviation = primary_color.deviation(bar_color);
		println!("avatar bar color of {i} has a deviation of {deviation} and color of {bar_color:?}");
		if deviation >= 0.05 {
			break;
		}
		
		player_count = i + 1;
	}
	
	println!("avatarcount is {player_count}");
	
	let secondary_color = image.sub_image(AVATAR_START + (AVATAR_SIZE + AVATAR_SPACING) * player_count + offset_secondary, TEXT_Y, TEXT_CHECK_WIDTH, TEXT_CHECK_HEIGHT).average_color();
	
	Theme {
		primary: primary_color,
		secondary: secondary_color,
	}
}