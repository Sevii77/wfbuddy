use std::{fs::File, io::{BufReader, BufWriter}};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
	pub app_id: String,
	pub log_path: String,
}

impl Config {
	pub fn load() -> Self {
		let Ok(file) = File::open(dirs::config_dir().unwrap().join("WFBuddy").join("config.json")) else {return Default::default()};
		serde_json::from_reader(BufReader::new(file)).unwrap_or_default()
	}
	
	pub fn save(&self) {
		let dir_path = dirs::config_dir().unwrap().join("WFBuddy");
		_ = std::fs::create_dir_all(&dir_path);
		let config_path = dir_path.join("config.json");
		let writer = BufWriter::new(File::create(config_path).unwrap());
		serde_json::to_writer(writer, self).unwrap()
	}
}

impl Default for Config {
	fn default() -> Self {
		Self {
			// TODO: check if same on windows
			app_id: "steam_app_230410".to_string(),
			
			#[cfg(unix)]
			log_path: dirs::home_dir().unwrap().join(".steam/steam/steamapps/compatdata/230410/pfx/drive_c/users/steamuser/AppData/Local/Warframe/EE.log").to_string_lossy().to_string(),
			#[cfg(windows)]
			log_path: dirs::cache_dir().unwrap().join("Local/Warframe/EE.log").to_string_lossy().to_string(),
		}
	}
}