use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use gdk_pixbuf::Pixbuf;
use gtk::{Builder, ListBoxRow};
use gtk::prelude::*;
use itertools::Itertools;
use crate::data::{
	db_storage::Storage,
	db_types::ArmorClass,
};
use crate::data::db_types::ArmorRank;
use crate::ui::{
	*,
	pages::{set_image_scaled, SMALL_SIZE_ICON},
};
use strum::EnumCount;

pub(crate) struct ArmorsPage {
	rank_tabs: [gtk::ListBox; ArmorRank::COUNT],
	rank_switches: [gtk::Switch; ArmorRank::COUNT],
	images: Rc<HashMap<String, Pixbuf>>,
}

impl ArmorsPage {
	pub fn new(builder: &Builder, images: Rc<HashMap<String, Pixbuf>>) -> Self {
		let rank_tabs = [
			builder.object("lr list").unwrap(),
			builder.object("hr list").unwrap(),
			builder.object("mr list").unwrap(),
		];
		let rank_switches = [
			builder.object("lr switch").unwrap(),
			builder.object("hr switch").unwrap(),
			builder.object("mr switch").unwrap(),
		];
		ArmorsPage {
			rank_tabs,
			rank_switches,
			images,
		}
	}

	pub fn show(self: &Rc<Self>, storage: &Rc<Storage>, dynamic_storage: &Rc<DynamicStorage>) {
		for rank in ArmorRank::iter() {
			let switch = &self.rank_switches[rank as usize];
			let copy = Rc::clone(self);
			switch.connect_changed_active(move |switch| {
				println!("changed all {} to status: {}", rank.to_string(), switch.state());
				copy.rank_tabs[rank as usize].foreach(|w| {
					let gtkbox: gtk::Box = ((w.downcast_ref::<ListBoxRow>().unwrap()).child().unwrap()).downcast_ref::<gtk::Box>().unwrap().clone();
					for elem in gtkbox.children() {
						if let Some(child_switch) = elem.downcast_ref::<gtk::Switch>() {
							child_switch.set_state(switch.state())
						}
					}
				});
			});
		}
		for set in storage.sets.iter().sorted_by(|a, b| { a.id.cmp(&b.id) }) {
			let builder = get_builder("res/gui/set box.glade".to_string());
			let set_row: gtk::ListBoxRow = builder.object("row").unwrap();
			let name: gtk::Label = builder.object("name").unwrap();
			name.set_text(&set.name);
			for piece in ArmorClass::iter() {
				let image: gtk::Image = builder.object(piece.to_string().as_str()).unwrap();
				if set.get_armor(piece).is_some() {
					set_image_scaled(&image, piece.to_string().as_str(), SMALL_SIZE_ICON, &self.images);
				} else {
					set_image_scaled(&image, "armor missing", SMALL_SIZE_ICON, &self.images);
				}
			}
			self.rank_tabs[set.rank as usize].insert(&set_row, -1);

			let enable_switch : gtk::Switch = builder.object("enable").unwrap();
			let dynamic_storage_copy = Rc::clone(&dynamic_storage);
			let set_copy = Arc::clone(set);
			enable_switch.connect_changed_active(move |switch| {
				dynamic_storage_copy.set_armors_set(set_copy.clone(), switch.state());
				println!("{} status: {}", set_copy.name, switch.state());
			});
		}
	}
}