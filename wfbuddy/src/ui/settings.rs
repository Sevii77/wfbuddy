use crate::ui::ext::UiExt;

pub fn ui(ui: &mut egui::Ui, modules: &mut [Box<dyn crate::module::Module>]) {
	let mut config = crate::config();
	let mut changed = false;
	
	changed |= ui.combo_cached(&mut config.app_id, "Warframe Window ID", || {
		println!("fetching windows!");
		xcap::Window::all().unwrap().into_iter().filter_map(|v| v.app_name().ok()).collect()
	});
	
	changed |= ui.text_edit_singleline(&mut config.log_path).changed();
	
	for module in modules {
		ui.spacer();
		changed |= module.ui_settings(ui);
	}
	
	if changed {
		config.save();
	}
}