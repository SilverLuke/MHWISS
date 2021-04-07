use gtk::Builder;
use gtk::prelude::*;
use crate::ui::ui::Ui;
use std::sync::Arc;
use crate::datatypes::forge::Forge;
use itertools::Itertools;

pub(crate) struct ArmorsPage {
	rank_set: [gtk::ListBox; 3],
}

impl ArmorsPage {
	pub fn new(builder: &Builder) -> Self {
		let rank_set = [
			builder.get_object("lr list").unwrap(),
			builder.get_object("hr list").unwrap(),
			builder.get_object("mr list").unwrap(),
		];
		ArmorsPage {
			rank_set,
		}
	}

	pub fn show(&self, forge: &Arc<Forge>) {
		for set in forge.sets.values().sorted_by(|a, b| { a.id.cmp(&b.id) }) {
			let builder = Ui::get_builder("gui/set box.glade".to_string());
			let set_row: gtk::ListBoxRow = builder.get_object("row").unwrap();
			let name: gtk::Label = builder.get_object("name").unwrap();

			name.set_text(&set.name);
			self.rank_set[set.rank as usize].insert(&set_row, -1);
		}
	}
}