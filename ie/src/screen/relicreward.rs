use std::sync::LazyLock;
use crate::{Image, Mask, OwnedImage, OwnedMask, Theme};

pub struct Rewards {
	pub timer: u32,
	pub rewards: Vec<Reward>,
}

pub struct Reward {
	pub name: String,
	pub owned: u32,
}

/// Expects an image with a height of 1080
pub(crate) fn get_rewards(image: Image, theme: Theme, ocr: &crate::ocr::Ocr) -> Rewards {
	const CRAFTED_AREA_SIZE: u32 = 32;
	const NAME_AREA_SIZE: u32 = 70;
	const TIMER_Y: u32 = 135;
	const TIMER_W: u32 = 64;
	const TIMER_H: u32 = 64;
	const OWNED_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| regex::Regex::new(r"(?<amount>\d+)?\s*(?:Owned|Crafted)").unwrap());
	
	let rewards = get_reward_subimages(image)
		.into_iter()
		.map(|image| Reward {
			name: image.trimmed_bottom(NAME_AREA_SIZE).get_text(theme, ocr),
			owned: OWNED_REGEX
				.captures(&image.trimmed_top(CRAFTED_AREA_SIZE).get_text(theme, ocr))
				.map(|v| v
					.name("amount")
					.map(|v| v.as_str().parse::<u32>().ok())
					.flatten()
					.unwrap_or(1))
				.unwrap_or(0),
		})
		.collect();
	
	let timer = crate::util::DIGIT_REGEX
		.captures(&image
			.trimmed_centerh(TIMER_W)
			.sub_image(0, TIMER_Y, TIMER_W, TIMER_H)
			.get_text(Theme::WHITE, ocr))
		.map(|v| v
			.name("digits")
			.map(|v| v.as_str().parse::<u32>().ok())
			.flatten())
		.flatten()
		.unwrap_or(10);
	
	Rewards{timer, rewards}
}

/// Expects an image with a height of 1080
pub(crate) fn get_selected(image: Image, theme: Theme) -> u32 {
	let rewards = get_reward_subimages(image);
	
	let mut picked = 0;
	let mut deviation = 1.0;
	for (i, image) in rewards.iter().enumerate() {
		let dev = image.sub_image(image.width() - 12, 0, 12, 12).average_color().deviation(theme.secondary);
		println!("pick check dev {dev}");
		if dev < deviation {
			picked = i as u32;
			deviation = dev;
		}
	}
	
	picked
}

fn get_reward_subimages<'a>(image: Image<'a>) -> Vec<Image<'a>> {
	const REWARD_SIZE: u32 = 235;
	const REWARD_SPACING: u32 = 8;
	const REWARD_Y: u32 = 225;
	
	let count = reward_count(image);
	println!("rewardcount is {count}");
	
	let area_width = count * REWARD_SIZE + (count - 1) * REWARD_SPACING;
	let image = image.trimmed_centerh(area_width);
	
	let mut images = Vec::with_capacity(count as usize);
	for i in 0..count {
		let offset = (REWARD_SIZE + REWARD_SPACING) * i;
		images.push(image.sub_image(offset, REWARD_Y, REWARD_SIZE, REWARD_SIZE));
	}
	
	images
}

static ICON_COMMON: LazyLock<(OwnedImage, OwnedMask)> = LazyLock::new(|| {println!("1"); crate::OwnedImage::from_png_mask(include_bytes!("../asset/icon_common.png"), 250).unwrap()});
static ICON_UNCOMMON: LazyLock<(OwnedImage, OwnedMask)> = LazyLock::new(|| {println!("2"); crate::OwnedImage::from_png_mask(include_bytes!("../asset/icon_uncommon.png"), 250).unwrap()});
static ICON_RARE: LazyLock<(OwnedImage, OwnedMask)> = LazyLock::new(|| {println!("3"); crate::OwnedImage::from_png_mask(include_bytes!("../asset/icon_rare.png"), 250).unwrap()});

// Gets the amount of rewards there are, not always equal to the amount of people
// in the party if someone forgot to select a relic.
fn reward_count(image: Image) -> u32 {
	const REWARDS_AREA_WIDTH: u32 = 962;
	const RARITY_ICON_OFFSET: u32 = 242;
	const RARITY_ICON_Y: u32 = 459;
	const RARITY_ICON_SIZE: u32 = 40;
	
	let image = image.trimmed_centerh(REWARDS_AREA_WIDTH);
	
	fn check_icon(image: Image, x: u32) -> bool {
		for icon in [&ICON_COMMON, &ICON_UNCOMMON, &ICON_RARE] {
			for jitter_y in -1..=1 { // sub pixel shenenigans
				for jitter_x in -1..=1 {
					let sub = image.sub_image((x as isize + jitter_x) as u32, (RARITY_ICON_Y as i32 + jitter_y) as u32, RARITY_ICON_SIZE, RARITY_ICON_SIZE);
					// let deviation = sub.average_color_masked(Mask(&icon.1.0)).deviation(icon.0.as_image().average_color_masked(Mask(&icon.1.0)));
					let deviation = sub.average_deviation_masked(icon.0.as_image(), Mask(&icon.1.0));
					println!("icon check deviation was {deviation}");
					if deviation <= 25.0 {
						return true;
					}
				}
			}
		}
		
		false
	}
	
	println!("odd");
	let is_odd = check_icon(image, REWARDS_AREA_WIDTH / 2 - RARITY_ICON_SIZE / 2);
	println!("many");
	let is_many = check_icon(image, REWARDS_AREA_WIDTH / 2 + RARITY_ICON_OFFSET + if is_odd {0} else {RARITY_ICON_OFFSET / 2} - RARITY_ICON_SIZE / 2);
	match (is_odd, is_many) {
		(true, false) => 1,
		(false, false) => 2,
		(true, true) => 3,
		(false, true) => 4,
	}
}