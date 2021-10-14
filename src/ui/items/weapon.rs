use std::collections::HashMap;
use std::rc::Rc;

use gdk_pixbuf::Pixbuf;
use gtk::Builder;
use gtk::prelude::{BuilderExtManual, ImageExt, LabelExt, WidgetExt};

use crate::data::db_types::weapon::Weapon;
use crate::data::mutable::attached_decorations::AttachedDecorations;
use crate::ui::items::slots::GtkSlot;
use crate::ui::items::UI;

pub struct GtkWeapon {
	name: gtk::Label,
	pub image: gtk::Image,
	attack: gtk::Label,
	affinity: gtk::Label,
	element: [gtk::Label; 2],
	skill: gtk::Label,
	slots: Vec<GtkSlot>,
	images: Rc<HashMap<String, Pixbuf>>,
}

impl GtkWeapon {
	pub fn new(builder: &Builder, images: Rc<HashMap<String, Pixbuf>>) -> Self {
		let mut slots = Vec::with_capacity(3);
		for i in 0..3 {
			let name = format!("weapon slot {}", i);
			let image = format!("weapon slot image {}", i);
			slots.push(GtkSlot::new(&builder, name.as_str(), image.as_str(), Rc::clone(&images)));
		}
		GtkWeapon {
			name: builder.object("weapon name").unwrap(),
			image: builder.object("weapon image").unwrap(),
			attack: builder.object("weapon attack").unwrap(),
			affinity: builder.object("weapon affinity").unwrap(),
			element: [builder.object("weapon element 1").unwrap(), builder.object("weapon element 2").unwrap()],
			skill: builder.object("weapon skill").unwrap(),
			slots,
			images,
		}
	}
}

impl UI<AttachedDecorations<Weapon>> for GtkWeapon {
	fn update(&self, piece: &Option<AttachedDecorations<Weapon>>) {
		if let Some(weapon) = piece {
			self.show(weapon);
		} else {
			self.empty();
		}
	}

	fn empty(&self) {
		self.image.set_from_pixbuf(self.images.get("weapon empty"));
		self.name.set_text("-");
		self.attack.set_text("-");
		self.affinity.set_text("-");
		self.skill.set_text("-");
		self.element[0].set_text("");
		self.element[1].set_text("");
		for slot in self.slots.iter() {
			slot.empty(0);
		}
	}

	fn show(&self, item: &AttachedDecorations<Weapon>) {
		let weapon = item.get_item();
		self.image.set_from_pixbuf(self.images.get(format!("{}", weapon.class.to_string()).as_str()));
		self.name.set_text(weapon.name.as_str());
		for skill_level in weapon.skill.iter() {
			self.skill.set_text(skill_level.get_skill().name.as_str());
			self.skill.show();
		}
		for (i, _size) in weapon.slots.iter().enumerate() {
			self.slots[i].empty(0);
		}
	}
}