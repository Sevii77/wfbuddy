use crate::{logwatcher::{LogWatcher, LogWatcherStatus}, module::{self, Module}};

mod ext;
pub use ext::UiExt;
mod settings;

pub struct WFBuddy {
	modules: Vec<Box<dyn Module>>,
	watcher: LogWatcher,
	tab: &'static str,
}

impl WFBuddy {
	pub fn new(_cc: &eframe::CreationContext) -> Self {
		let data = std::sync::Arc::new(data::Data::populated());
		let watcher = LogWatcher::watch(&crate::config().log_path).unwrap();
		
		Self {
			modules: vec![
				Box::new(module::RelicReward::new(&watcher, data.clone())),
			],
			watcher,
			tab: "Home",
		}
	}
	
	fn ui(&mut self, ui: &mut egui::Ui) {
		match self.watcher.status() {
			LogWatcherStatus::Watching =>
				_ = ui.label("Logwatcher: Active"),
			
			LogWatcherStatus::Stopped =>
				_ = ui.label("Logwatcher: Stopped"),
			
			LogWatcherStatus::Failed(error) =>
				_ = ui.label(format!("Logwatcher: Error {error:#?}")),
		}
		
		ui.horizontal(|ui| {
			if ui.selectable_label(self.tab == "Home", "Home").clicked() {
				self.tab = "Home";
			}
			
			if ui.selectable_label(self.tab == "Settings", "Settings").clicked() {
				self.tab = "Settings";
			}
			
			for module in &mut self.modules {
				if ui.selectable_label(self.tab == module.name(), module.name()).clicked() {
					self.tab = module.name();
				}
			}
		});
		
		ui.separator();
		
		match self.tab {
			"Home" => {
				for module in &mut self.modules {
					if module.ui_important(ui) {
						ui.separator();
					}
				}
			}
			
			"Settings" => {
				settings::ui(ui, &mut self.modules);
			}
			
			tab => {
				for module in &mut self.modules {
					if module.name() == tab {
						module.ui(ui);
						break;
					}
				}
			}
		}
	}
}

impl eframe::App for WFBuddy {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		// println!("tick");
		for module in &mut self.modules {
			module.tick();
		}
		
		egui::CentralPanel::default().show(&ctx, |ui| self.ui(ui));
		
		// https://github.com/emilk/egui/issues/5113
		// https://github.com/emilk/egui/pull/7775
		ctx.request_repaint();
	}
}