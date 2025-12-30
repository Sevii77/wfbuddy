use std::collections::{HashMap, HashSet};

mod schema;

#[derive(Debug)]
pub struct Data {
	pub platinum_values: HashMap<String, f32>,
	pub ducat_values: HashMap<String, u32>,
	pub relic_items: HashSet<String>,
	pub vaulted_items: HashSet<String>,
}

impl Data {
	pub fn populated() -> Self {
		let items = ureq::get(schema::items::URL)
			.call()
			.unwrap()
			.body_mut()
			.read_json::<schema::items::Items>()
			.unwrap();
		
		let ducats = ureq::get(schema::ducats::URL)
			.call()
			.unwrap()
			.body_mut()
			.read_json::<schema::ducats::Ducats>()
			.unwrap();
		
		let name_map = items.data
			.iter()
			.map(|v| (v.id.clone(), v.i18n.en.name.clone()))
			.collect::<HashMap<_, _>>();
		
		let mut s = Self {
			platinum_values: ducats.payload.previous_hour
				.iter()
				.map(|v| (name_map[&v.item].clone(), v.wa_price))
				.collect(),
			
			ducat_values: ducats.payload.previous_hour
				.iter()
				.map(|v| (name_map[&v.item].clone(), v.ducats))
				.collect(),
			
			relic_items: ducats.payload.previous_hour
				.iter()
				.map(|v| name_map[&v.item].clone())
				.collect(),
			
			vaulted_items: HashSet::new(), //TODO
			// vaulted_items: items.data
			// 	.iter()
			// 	.filter_map(|v| if v. {Some(fix_relic_reward_name(name))} else {None})
			// 	.collect(),
		};
		
		s.platinum_values.insert("Forma Blueprint".to_string(), (350.0f32 / 3.0).floor() * 0.1);
		s.relic_items.insert("Forma Blueprint".to_string());
		s.platinum_values.insert("2 X Forma Blueprint".to_string(), (350.0f32 / 3.0).floor() * 0.2);
		s.relic_items.insert("2 X Forma Blueprint".to_string());
		
		s
	}
	
	/// Attempts to find the closest item name from a dirty ocr string
	pub fn find_item_name(&self, name: &str) -> String {
		let name = name.trim_ascii();
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