use crate::ui::items::slots::GtkSlot;
use gtk::{Builder, ImageExt, LabelExt, WidgetExt};
use gtk::prelude::BuilderExtManual;
use crate::datatypes::charm::Charm;
use crate::ui::items::UI;
use std::collections::HashMap;
use gdk_pixbuf::Pixbuf;
use std::collections::hash_map::RandomState;
use std::rc::Rc;
use std::sync::Arc;

pub struct GtkCharm {
	name: gtk::Label,
	pub image: gtk::Image,
	skill: [gtk::Label; 2],
	images: Rc<HashMap<String, Pixbuf>>
}

impl GtkCharm {
	pub fn new(builder: &Builder, images: Rc<HashMap<String, Pixbuf>>) -> Self {
		GtkCharm {
			name: builder.get_object("charm name").unwrap(),
			image: builder.get_object("charm image").unwrap(),
			skill: [builder.get_object("charm skill 1").unwrap(),builder.get_object("charm skill 2").unwrap()],
			images: images,
		}
	}
}

impl UI<Arc<Charm>> for GtkCharm {
	fn update(&self, piece: &Option<Arc<Charm>>) {
		if let Some(weapon) = piece {
			self.show(weapon);
		} else {
			self.empty();
		}
	}

	fn empty(&self) {
		self.image.set_from_pixbuf(self.images.get("charm empty"));
		self.name.set_text("-");
		self.skill[0].set_text("-");
		self.skill[1].set_text("-");
	}

	fn show(&self, item: &Arc<Charm>) {
		self.image.set_from_pixbuf(self.images.get("charm"));
		self.name.set_text(item.name.as_str());
		for (i, charm_skill) in item.skills.get_skills().enumerate() {
			self.skill[i].set_text(format!("{} {}", charm_skill.skill.name, charm_skill.level).as_str());
			self.skill[i].show();
		}
	}
}