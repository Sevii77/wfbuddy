use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FilteredItems {
	#[serde(rename = "eqmt")]
	pub sets: HashMap<String, Set>,
	// pub ignored_items: HashMap<String, ()>,
}

#[derive(Deserialize)]
pub struct Set {
	// #[serde(rename = "type")]
	// pub typ: String,
	// pub vaulted: bool,
	pub parts: HashMap<String, Part>,
}

#[derive(Deserialize)]
pub struct Part {
	// pub count: u32,
	#[serde(default)]
	pub ducats: u32,
	pub vaulted: bool,
}