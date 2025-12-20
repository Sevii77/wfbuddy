use crate::{Color, Image, Mask};

pub struct Reward {
	pub name: String,
	pub owned: u32,
}

/// Expects an image with a height of 1080
pub fn get_rewards(image: Image) -> Vec<Reward> {
	const CRAFTED_AREA_SIZE: u32 = 32;
	const NAME_AREA_SIZE: u32 = 70;
	const OWNED_REGEX: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| regex::Regex::new(r"(?<amount>\d+)?\s*(?:Owned|Crafted)").unwrap());
	
	get_reward_subimages(image)
		.into_iter()
		.map(|image| Reward {
			name: image.trimmed_bottom(NAME_AREA_SIZE).get_text(),
			owned: OWNED_REGEX
				.captures(&image.trimmed_top(CRAFTED_AREA_SIZE).get_text())
				.map(|v| v
					.name("amount")
					.map(|v| v.as_str().parse::<u32>().ok())
					.flatten()
					.unwrap_or(1))
				.unwrap_or(0),
		})
		.collect()
}

/// Expects an image with a height of 1080
pub fn get_selected(image: Image) -> u32 {
	const REWARDS_TEXT_OFFSET: u32 = 348;
	
	let theme = crate::theme::from_party_header_text(image, REWARDS_TEXT_OFFSET);
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

const RARITY_ICON_SIZE: u32 = 16;

const COLOR_COMMON: Color = Color::new(190, 141, 121);
const MASK_COMMON: Mask = Mask(&[
	0b00000000, 0b00000000,
	0b00000000, 0b00000000,
	0b00000001, 0b00000000,
	0b00000011, 0b00000000,
	0b00000111, 0b00000000,
	0b00001111, 0b00000000,
	0b00011111, 0b00000000,
	0b00011111, 0b00000000,
	0b00011111, 0b00000000,
	0b00011111, 0b00000000,
	0b00011111, 0b00000000,
	0b00001111, 0b00000000,
	0b00000111, 0b00000000,
	0b00000011, 0b00000000,
	0b00000001, 0b00000000,
	0b00000000, 0b00000000,
]);

const COLOR_UNCOMMON: Color = Color::new(207, 207, 207);
const MASK_UNCOMMON: Mask = Mask(&[
	0b00000011, 0b00000000,
	0b00000011, 0b00000000,
	0b00000011, 0b00000000,
	0b00000111, 0b00000000,
	0b00000111, 0b00000000,
	0b00001111, 0b00000000,
	0b00001111, 0b00000000,
	0b00001111, 0b00000000,
	0b00001111, 0b00000000,
	0b00001111, 0b00000000,
	0b10000111, 0b00000001,
	0b10000111, 0b00000001,
	0b00000011, 0b00000000,
	0b00000011, 0b00000000,
	0b00000011, 0b00000000,
	0b00000001, 0b00000000,
]);

const COLOR_RARE: Color = Color::new(231, 211, 140);
const MASK_RARE: Mask = Mask(&[
	0b00000110, 0b00000000,
	0b00000110, 0b00000000,
	0b00000110, 0b00000001,
	0b10001110, 0b00000001,
	0b10001110, 0b00000001,
	0b10001110, 0b00000001,
	0b00001110, 0b00000000,
	0b00001110, 0b00000000,
	0b00001110, 0b00000000,
	0b00000110, 0b00000000,
	0b00000110, 0b00000000,
	0b00000110, 0b00000000,
	0b00000010, 0b00000000,
	0b00000010, 0b00000000,
	0b00000000, 0b00000000,
	0b00000000, 0b00000000,
]);

// Gets the amount of rewards there are, not always equal to the amount of people
// in the party if someone forgot to select a relic.
// the log does contain `ProjectionRewardChoice.lua: Missing icon data!` that seems to match the amount
// of rewards after a quick glance but seems unreliable (maybe investigate more in the future?)
fn reward_count(image: Image) -> u32 {
	const REWARDS_AREA_WIDTH: u32 = 962;
	const RARITY_ICON_OFFSET: u32 = 242;
	const RARITY_ICON_Y: u32 = 471;
	const RARITY_ICON_EVEN_OFFSET_START: u32 = 111;
	
	let image = image.trimmed_centerh(REWARDS_AREA_WIDTH);
	
	fn check_icon(image: Image, x: u32) -> bool {
		for (mask, color) in &[(MASK_COMMON, COLOR_COMMON), (MASK_UNCOMMON, COLOR_UNCOMMON), (MASK_RARE, COLOR_RARE)] {
			for jitter in -1..=1 { // sub pixel shenenigans
				let sub = image.sub_image((x as isize + jitter) as u32, RARITY_ICON_Y, RARITY_ICON_SIZE, RARITY_ICON_SIZE);
				let deviation = sub.averate_color_masked(mask).deviation(*color);
				println!("icon check deviation was {deviation}");
				if deviation <= 0.2 {
					return true;
				}
			}
		}
		
		false
	}
	
	let is_odd = check_icon(image, REWARDS_AREA_WIDTH / 2 - RARITY_ICON_SIZE / 2);
	let is_many = check_icon(image, REWARDS_AREA_WIDTH / 2 + RARITY_ICON_OFFSET + if is_odd {0} else {RARITY_ICON_EVEN_OFFSET_START} - RARITY_ICON_SIZE / 2);
	match (is_odd, is_many) {
		(true, false) => 1,
		(false, false) => 2,
		(true, true) => 3,
		(false, true) => 4,
	}
}