use gtk::Builder;
use gtk::prelude::*;
use crate::ui::ui::Ui;
use std::sync::Arc;
use crate::datatypes::forge::Forge;
use itertools::Itertools;
use crate::datatypes::types::ArmorClass;
use gdk_pixbuf::Pixbuf;
use std::collections::HashMap;
use std::rc::Rc;
use crate::ui::SMALL_SIZE_ICON;

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

	pub fn show(&self, forge: &Arc<Forge>) {
		for set in forge.sets.values().sorted_by(|a, b| { a.id.cmp(&b.id) }) {
			let builder = Ui::get_builder("res/gui/set box.glade".to_string());
			let set_row: gtk::ListBoxRow = builder.get_object("row").unwrap();
			let name: gtk::Label = builder.get_object("name").unwrap();
			name.set_text(&set.name);
			for piece in ArmorClass::iterator() {
				let image: gtk::Image = builder.get_object(piece.to_string().as_str()).unwrap();
				if set.set.get(*piece as usize).unwrap().is_none() {
					Ui::set_image_scaled(&image, "armor missing", SMALL_SIZE_ICON, &self.images);
				} else {
					Ui::set_image_scaled(&image, piece.to_string().as_str(), SMALL_SIZE_ICON, &self.images);
				}
			}
			self.rank_set[set.rank as usize].insert(&set_row, -1);
		}
	}
}