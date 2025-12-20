mod relicreward;
pub use relicreward::RelicReward;

pub trait Module {
	fn name(&self) -> &'static str;
	
	fn ui(&mut self, ui: &mut egui::Ui);
	
	fn ui_settings(&mut self, ui: &mut egui::Ui) -> bool;
	
	#[allow(unused_variables)]
	fn ui_important(&mut self, ui: &mut egui::Ui) -> bool {false}
	
	fn tick(&mut self) {}
}