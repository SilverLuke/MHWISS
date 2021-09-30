use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use gdk_pixbuf::Pixbuf;
use gtk::Builder;
use gtk::prelude::*;
use itertools::Itertools;
use crate::data::{
	db_storage::Storage,
	db_types::ArmorClass,
};
use crate::ui::{
	*,
	pages::{set_image_scaled, SMALL_SIZE_ICON},
};

pub(crate) struct ArmorsPage {
	rank_set: [gtk::ListBox; 3],
	images: Rc<HashMap<String, Pixbuf>>,
}

impl ArmorsPage {
	pub fn new(builder: &Builder, images: Rc<HashMap<String, Pixbuf>>) -> Self {
		let rank_set = [
			builder.get_object("lr list").unwrap(),
			builder.get_object("hr list").unwrap(),
			builder.get_object("mr list").unwrap(),
		];
		ArmorsPage {
			rank_set,
			images,
		}
	}

	pub fn show(&self, storage: &Arc<Storage>) {
		for set in storage.sets.iter().sorted_by(|a, b| { a.id.cmp(&b.id) }) {
			let builder = get_builder("res/gui/set box.glade".to_string());
			let set_row: gtk::ListBoxRow = builder.get_object("row").unwrap();
			let name: gtk::Label = builder.get_object("name").unwrap();
			name.set_text(&set.name);
			for piece in ArmorClass::iter() {
				let image: gtk::Image = builder.get_object(piece.to_string().as_str()).unwrap();
				if set.get_armor(piece).is_some() {
					set_image_scaled(&image, piece.to_string().as_str(), SMALL_SIZE_ICON, &self.images);
				} else {
					set_image_scaled(&image, "armor missing", SMALL_SIZE_ICON, &self.images);
				}
			}
			self.rank_set[set.rank as usize].insert(&set_row, -1);
		}
	}
}