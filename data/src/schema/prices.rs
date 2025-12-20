use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
pub struct Prices(pub Vec<Item>);

#[derive(Deserialize)]
pub struct Item {
	pub name: String,
	// #[serde(deserialize_with = "f32::deserialize")]
	// pub yesterday_vol: f32,
	// #[serde(deserialize_with = "f32::deserialize")]
	// pub today_vol: f32,
	#[serde(deserialize_with = "str_f32")]
	pub custom_avg: f32,
}

fn str_f32<'de, D>(deserializer: D) -> Result<f32, D::Error> where
D: Deserializer<'de> {
	let s = String::deserialize(deserializer)?;
	s.parse::<f32>().map_err(|_| serde::de::Error::invalid_value(serde::de::Unexpected::Str(&s), &"a valid number"))
}