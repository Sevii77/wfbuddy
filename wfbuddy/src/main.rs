use std::sync::{Arc, LazyLock, Mutex};

pub mod util;
pub mod capture;
mod logwatcher;
mod module;
mod ui;
pub use ui::UiExt;
mod config;

static CONFIG: LazyLock<Arc<Mutex<config::Config>>> = LazyLock::new(|| Arc::new(Mutex::new(config::Config::load())));
pub fn config() -> std::sync::MutexGuard<'static, config::Config> {
	CONFIG.lock().unwrap()
}

fn main() {
	eframe::run_native("WFBuddy", Default::default(), Box::new(|cc| Ok(Box::new(ui::WFBuddy::new(cc))))).unwrap();
}