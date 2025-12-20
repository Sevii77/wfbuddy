use std::{collections::BTreeMap, time::{Duration, Instant}};
use crate::{logwatcher::{EventReceiver, LogWatcher}, UiExt};

// const MAX_TIMER_DELAY_MS: u64 = 100;
// const PRE_REWARDS_CHECK_S: f32 = 1.0;
const CHECK_SELECTED_TIMER_MS: u64 = 14000;

// 6728.482 Script [Info]: ProjectionRewardChoice.lua: Got rewards
// 6728.483 Script [Info]: ProjectionsCountdown.lua: Initialize timer nil    15

pub struct RelicReward {
	data: std::sync::Arc<data::Data>,
	
	rewards_rs: EventReceiver,
	// timer_rs: EventReceiver,
	// last_rewards: Instant,
	check_selected: Option<Instant>,
	
	current_rewards: Vec<Reward>,
	selected_rewards: BTreeMap<String, u32>,
}

impl RelicReward {
	pub fn new(watcher: &LogWatcher, data: std::sync::Arc<data::Data>) -> Self {
		let (tx, rewards_rs) = std::sync::mpsc::channel();
		watcher.watch_event(regex::Regex::new(r"ProjectionRewardChoice\.lua: Got rewards$").unwrap(), tx);
		
		// let (tx, timer_rs) = std::sync::mpsc::channel();
		// watcher.watch_event(regex::Regex::new(r"ProjectionsCountdown\.lua: Initialize timer.+ (?<delay>\d+)$").unwrap(), tx);
		
		Self {
			data,
			
			rewards_rs,
			// timer_rs,
			// last_rewards: Instant::now(),
			check_selected: None,
			
			current_rewards: Vec::new(),
			selected_rewards: BTreeMap::new(),
		}
	}
}

impl super::Module for RelicReward {
	fn name(&self) -> &'static str {
		"Relic Rewards"
	}
	
	fn ui(&mut self, ui: &mut egui::Ui) {
		self.ui_important(ui);
	}
	
	fn ui_settings(&mut self, ui: &mut egui::Ui) -> bool {
		ui.label("Relic Rewards");
		ui.label("TODO");
		
		false
	}
	
	fn ui_important(&mut self, ui: &mut egui::Ui) -> bool {
		let reward_count = self.current_rewards.len();
		let selected_count = self.selected_rewards.len();
		if reward_count == 0 && selected_count == 0 {return false}
		
		ui.columns(reward_count, |uis| {
			for (i, ui) in uis.into_iter().enumerate() {
				let reward = &self.current_rewards[i];
				ui.label(&reward.name);
				ui.label(format!("Platinum: {}", reward.platinum));
				ui.label(format!("Ducats: {}", reward.ducats));
				
				if reward.owned > 0 {
					ui.label(format!("Owned: {}", reward.owned + self.selected_rewards.get(&reward.name).map_or(0, |v| *v)));
				} else {
					ui.label("");
				}
				
				if reward.vaulted {
					ui.label("Vaulted");
				}
			}
		});
		
		if selected_count > 0 {
			if reward_count > 0 {
				ui.spacer();
			}
			
			ui.label("Selected Rewards");
			ui.indent("selected", |ui| {
				for (item, amount) in &self.selected_rewards {
					ui.label(format!("{item} x{amount}"));
				}
			});
			
			ui.spacer();
			if ui.button("Clear Selected Rewards").clicked() {
				self.selected_rewards.clear();
			}
		}
		
		true
	}
	
	fn tick(&mut self) {
		// TODO: seperate thread mby so it doesnt block
		'rewards: {
			let Ok((timestamp, _cap)) = self.rewards_rs.try_recv() else {break 'rewards};
			println!("- checking rewards");
			
			let Some(mut img) = crate::capture::capture() else {break 'rewards};
			img.resize_h(1080);
			
			self.current_rewards = ie::screen::relicreward::get_rewards(img.as_image())
				.into_iter()
				.map(|reward| {
					let name = self.data.find_item_name(&reward.name);
					Reward {
						vaulted: self.data.vaulted_items.contains(&name),
						platinum: self.data.platinum_values.get(&name).map(|v| *v).unwrap_or_default(),
						ducats: self.data.ducat_values.get(&name).map(|v| *v).unwrap_or_default(),
						owned: reward.owned,
						name,
					}
				})
				.collect::<Vec<_>>();
			
			// self.last_rewards = timestamp;
			self.check_selected = Some(timestamp + Duration::from_millis(CHECK_SELECTED_TIMER_MS));
		}
		
		// 'timer: {
		// 	let Ok((timestamp, cap)) = self.timer_rs.try_recv() else {break 'timer};
		// 	if timestamp.duration_since(self.last_rewards) > Duration::from_millis(MAX_TIMER_DELAY_MS) {break 'timer}
		// 	if self.check_selected.is_some() {break 'timer}
		// 	let Some(delay) = cap.name("delay") else {break 'timer};
		// 	let Ok(delay) = delay.parse::<f32>() else {break 'timer};
		// 	println!("- timer for {delay} seconds");
		// 	self.check_selected = Some(self.last_rewards + Duration::from_secs_f32(delay - PRE_REWARDS_CHECK_S));
		// }
		
		'selected: {
			let Some(timestamp) = self.check_selected else {break 'selected};
			if timestamp > Instant::now() {break 'selected}
			println!("- checking selected");
			
			self.check_selected = None;
			
			let Some(mut img) = crate::capture::capture() else {break 'selected};
			img.resize_h(1080);
			
			let selected = ie::screen::relicreward::get_selected(img.as_image());
			let Some(reward) = self.current_rewards.get(selected as usize) else {break 'selected};
			println!("incrementing {} as the picked index was {selected}", reward.name);
			*self.selected_rewards.entry(reward.name.clone()).or_insert(0) += 1;
			self.current_rewards.clear();
		}
	}
}

struct Reward {
	name: String,
	vaulted: bool,
	platinum: f32,
	ducats: u32,
	owned: u32,
}