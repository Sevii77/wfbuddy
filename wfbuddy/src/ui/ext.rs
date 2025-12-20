use egui::{Ui, WidgetText};

pub trait UiExt {
	// https://github.com/Sevii77/aetherment/blob/master/aetherment/src/ui_ext/mod.rs#L57
	// maybe i should make a repo for egui extensions
	fn combo<S: Into<WidgetText>, S2: Into<WidgetText>>(&mut self, preview: S2, label: S, contents: impl FnOnce(&mut Ui));
	fn combo_id<S: Into<WidgetText>>(&mut self, preview: S, id: impl std::hash::Hash, contents: impl FnOnce(&mut Ui));
	fn combo_cached<S: Into<WidgetText>>(&mut self, selected: &mut String, label: S, contents: impl FnOnce() -> Vec<String>) -> bool;
	fn spacer(&mut self);
}

impl UiExt for Ui {
	fn combo<S: Into<WidgetText>, S2: Into<WidgetText>>(&mut self, preview: S2, label: S, contents: impl FnOnce(&mut Ui)) {
		egui::ComboBox::from_label(label)
			.height(300.0)
			.selected_text(preview)
			.show_ui(self, contents);
	}
	
	fn combo_id<S: Into<WidgetText>>(&mut self, preview: S, id: impl std::hash::Hash, contents: impl FnOnce(&mut Ui)) {
		egui::ComboBox::from_id_salt(id)
			.height(300.0)
			.selected_text(preview)
			.show_ui(self, contents);
	}
	
	fn combo_cached<S: Into<WidgetText>>(&mut self, selected: &mut String, label: S, contents_loader: impl FnOnce() -> Vec<String>) -> bool {
		let label: WidgetText = label.into();
		let id = self.id().with(label.text());
		let contents = self.data(|v| v.get_temp::<Vec<String>>(id));
		let contents_exist = contents.is_some();
		let mut changed = false;
		
		let r = egui::ComboBox::from_label(label)
			.height(300.0)
			.selected_text(&*selected)
			.show_ui(self, |ui| {
				let Some(contents) = contents else {return};
				for val in contents {
					if ui.selectable_label(val == *selected, &val).clicked() {
						*selected = val;
						changed = true;
					}
				}
			});
		
		if r.inner.is_none() && contents_exist {
			self.data_mut(|v| v.remove_temp::<Vec<String>>(id));
		} else if r.inner.is_some() && !contents_exist {
			self.data_mut(|v| v.insert_temp(id, contents_loader()));
		}
		
		changed
	}
	
	fn spacer(&mut self) {
		self.add_space(8.0);
	}
}