use crate::datatypes::types::*;
use gtk::{Builder, ImageExt, LabelExt, WidgetExt};
use gtk::prelude::BuilderExtManual;
use crate::datatypes::armor::Armor;
use std::collections::HashMap;
use gdk_pixbuf::Pixbuf;
use crate::ui::items::slots::*;
use crate::ui::items::UI;
use gio::FileEnumeratorExt;
use crate::datatypes::decoration::AttachedDecorations;
use std::collections::hash_map::RandomState;
use std::rc::Rc;

pub struct GtkArmour {
	name: gtk::Label,
	pub image: gtk::Image,
	class: ArmorClass,
	defence: gtk::Label,
	elements: Vec<gtk::Label>,
	skill: [gtk::Label; 2],
	slots: Vec<GtkSlot>,
	images: Rc<HashMap<String, Pixbuf>>,
}

impl GtkArmour {
	pub fn new(builder: &Builder, piece: ArmorClass,  images: Rc<HashMap<String, Pixbuf>>) -> Self {
		let piece_id = piece as u8;
		let iter = Element::iter_element();
		let mut elements = Vec::with_capacity(iter.len());
		for ele in iter {
			elements.push(builder.get_object(format!("{} {}", ele.to_string(), piece_id).as_str()).unwrap());
		}

		let mut slots = Vec::with_capacity(3);
		for id in 0..3 {
			let name = format!("slot {} {}", id, piece_id);  // TODO change to "armor slot {} {}"
			let image = format!("slot image {} {}", id, piece_id);
			slots.push(GtkSlot::new(&builder, name.as_str(), image.as_str(), Rc::clone(&images)));
		}
		GtkArmour {
			name: builder.get_object(&format!("name {}", piece_id)).unwrap(),
			image: builder.get_object(&format!("image {}", piece_id)).unwrap(),
			class: piece,
			defence: builder.get_object(&format!("defence {}", piece_id)).unwrap(),
			elements,
			skill: [builder.get_object(&format!("skill 0 {}", piece_id)).unwrap(), builder.get_object(&format!("skill 1 {}", piece_id)).unwrap()],
			slots,
			images,
		}
	}
}

impl UI<AttachedDecorations<Armor>> for GtkArmour {
	fn update(&self, piece: &Option<AttachedDecorations<Armor>>) {
		if let Some(piece) = piece {
			self.show_item(piece);
		} else {
			self.set_empty();
		}
	}

	fn set_empty(&self) {
		self.image.set_from_pixbuf(self.images.get(format!("{} empty", self.class.to_string()).as_str()));
		self.name.set_text("-");
		self.skill[0].set_text("");
		self.skill[1].set_text("");
		self.defence.set_text("");
		for element in self.elements.iter() {
			element.set_text("")
		}
		for slot in self.slots.iter() {
			slot.set_empty();
		}
	}

	fn show_item(&self, item: &AttachedDecorations<Armor>) {
		let piece = item.get_item();
		self.image.set_from_pixbuf(self.images.get(format!("{}", self.class.to_string()).as_str()));
		self.name.set_text(piece.name.as_str());
		for (i, (skill, lev)) in piece.skills.iter().enumerate() {
			self.skill[i].set_text(format!("{} {}", skill.name, lev).as_str());
			self.skill[i].show();
		}
		for (i, size) in piece.slots.iter().enumerate() {
			self.slots[i].update(&item.deco[i], *size);
		}
	}
}