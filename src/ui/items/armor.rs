use std::collections::HashMap;
use std::rc::Rc;
use gdk_pixbuf::Pixbuf;
use gtk::Builder;
use gtk::prelude::{BuilderExtManual, ImageExt, LabelExt, WidgetExt};
use crate::data::{
	mutable::attached_decorations::AttachedDecorations,
	db_types::{
		*,
		armor::Armor,
	}
};
use crate::ui::items::{
	slots::*,
	UI,
};


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
	pub fn new(builder: &Builder, piece: ArmorClass, images: Rc<HashMap<String, Pixbuf>>) -> Self {
		let piece_id = piece as u8;
		let iter = Element::iter_element();
		let mut elements = Vec::with_capacity(iter.len());
		for ele in iter {
			let msg = format!("{} {}", ele.to_string().to_lowercase(), piece_id);
			elements.push(builder.object(&msg).expect(&msg));
		}

		let mut slots = Vec::with_capacity(3);
		for id in 0..3 {
			let name = format!("armor slot {} {}", id, piece_id);
			let image = format!("armor slot image {} {}", id, piece_id);
			slots.push(GtkSlot::new(&builder, name.as_str(), image.as_str(), Rc::clone(&images)));
		}
		GtkArmour {
			name: builder.object(&format!("name {}", piece_id)).expect(&format!("UI do not found \"name {}\"", piece_id)),
			image: builder.object(&format!("image {}", piece_id)).expect(&format!("UI do not found \"image {}\"", piece_id)),
			class: piece,
			defence: builder.object(&format!("defence {}", piece_id)).expect(&format!("UI do not found \"defence {}\"", piece_id)),
			elements,
			skill: [
				builder.object(&format!("skill 0 {}", piece_id)).expect(&format!("UI do not found \"skill 0 {}\"", piece_id)),
				builder.object(&format!("skill 1 {}", piece_id)).expect(&format!("UI do not found \"skill 1 {}\"", piece_id))
			],
			slots,
			images,
		}
	}
}

impl UI<AttachedDecorations<Armor>> for GtkArmour {
	fn update(&self, piece: &Option<AttachedDecorations<Armor>>) {
		if let Some(piece) = piece {
			self.show(piece);
		} else {
			self.empty();
		}
	}

	fn empty(&self) {
		self.image.set_from_pixbuf(self.images.get(format!("{} empty", self.class.to_string()).as_str()));
		self.name.set_text("-");
		self.skill[0].set_text("");
		self.skill[1].set_text("");
		self.defence.set_text("");
		for element in self.elements.iter() {
			element.set_text("")
		}
		for slot in self.slots.iter() {
			slot.empty(0);
		}
	}

	fn show(&self, item: &AttachedDecorations<Armor>) {
		let piece = item.get_item();
		self.image.set_from_pixbuf(self.images.get(format!("{}", self.class.to_string()).as_str()));
		self.name.set_text(piece.name.as_str());
		for (i, armor_skill) in piece.skills.iter().enumerate() {
			self.skill[i].set_text(format!("{} {}", armor_skill.get_skill().name, armor_skill.get_level()).as_str());
			self.skill[i].show();
		}
		for (i, slot_size) in piece.slots.iter().enumerate() {
			self.slots[i].update(&item.get_deco(i), *slot_size);
		}
		self.defence.set_text(piece.defence[2].to_string().as_str());
		for (i, elem) in piece.elements.iter().enumerate() {
			self.elements[i].set_text(elem.to_string().as_str());
		}
	}
}