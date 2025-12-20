use std::collections::{HashMap, HashSet};

mod schema;

const FILTERED_ITEMS_URL: &str = "https://api.warframestat.us/wfinfo/filtered_items/";
const PRICES_URL: &str = "https://api.warframestat.us/wfinfo/prices/";

#[derive(Debug)]
pub struct Data {
	pub platinum_values: HashMap<String, f32>,
	pub ducat_values: HashMap<String, u32>,
	pub relic_items: HashSet<String>,
	pub vaulted_items: HashSet<String>,
}

impl Data {
	pub fn populated() -> Self {
		let filtered = ureq::get(FILTERED_ITEMS_URL)
			.call()
			.unwrap()
			.body_mut()
			.read_json::<schema::filtered_items::FilteredItems>()
			.unwrap();
		
		let prices = ureq::get(PRICES_URL)
			.call()
			.unwrap()
			.body_mut()
			.read_json::<schema::prices::Prices>()
			.unwrap();
		
		fn fix_relic_reward_name(s: &str) -> String {
			s.trim_end_matches("Blueprint").trim().to_owned()
		}
		
		let mut s = Self {
			platinum_values: prices.0
				.iter()
				.map(|v| (fix_relic_reward_name(&v.name), v.custom_avg))
				.collect(),
			
			ducat_values: filtered.sets
				.iter()
				.flat_map(|(_, v)| &v.parts)
				.map(|(name, v)| (fix_relic_reward_name(name), v.ducats))
				.collect(),
			
			relic_items: filtered.sets
				.iter()
				.flat_map(|(_, v)| &v.parts)
				.map(|(name, _)| fix_relic_reward_name(name))
				.collect(),
			
			vaulted_items: filtered.sets
				.iter()
				.flat_map(|(_, v)| &v.parts)
				.filter_map(|(name, v)| if v.vaulted {Some(fix_relic_reward_name(name))} else {None})
				.collect(),
		};
		
		s.platinum_values.insert("Forma".to_string(), (350.0f32 / 3.0).floor() * 0.1);
		s.relic_items.insert("Forma".to_string());
		s.platinum_values.insert("2 X Forma".to_string(), (350.0f32 / 3.0).floor() * 0.2);
		s.relic_items.insert("2 X Forma".to_string());
		
		s
	}
	
	/// Attempts to find the closest item name from a dirty ocr string
	pub fn find_item_name(&self, name: &str) -> String {
		let name = name.trim_end_matches(" Blueprint").trim();
		if self.relic_items.contains(name) {
			return name.to_owned()
		}
		
		let mut start = 0;
		while let Some(index) = name[start..].find(' ') {
			start += index + 1;
			let sub = &name[start..];
			if self.relic_items.contains(sub) {
				return sub.to_owned()
			}
		}
		
		let mut min_name = name;
		let mut min = usize::MAX;
		for item_name in self.relic_items.iter() {
			let lev = levenshtein::levenshtein(name, item_name);
			if lev < min {
				min_name = item_name.as_str();
				min = lev;
			}
		}
		
		min_name.to_string()
	}
}