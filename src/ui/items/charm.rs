use std::{
	collections::HashMap,
	rc::Rc,
	sync::Arc,
};
use gdk_pixbuf::Pixbuf;
use gtk::Builder;
use gtk::prelude::{BuilderExtManual, ImageExt, LabelExt, WidgetExt};
use crate::data::db_types::charm::Charm;
use crate::ui::items::UI;


pub struct GtkCharm {
	name: gtk::Label,
	pub image: gtk::Image,
	skill: [gtk::Label; 2],
	images: Rc<HashMap<String, Pixbuf>>
}

impl GtkCharm {
	pub fn new(builder: &Builder, images: Rc<HashMap<String, Pixbuf>>) -> Self {
		GtkCharm {
			name: builder.object("charm name").unwrap(),
			image: builder.object("charm image").unwrap(),
			skill: [builder.object("charm skill 1").unwrap(),builder.object("charm skill 2").unwrap()],
			images,
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
		for (i, charm_skill) in item.skills.iter().enumerate() {
			self.skill[i].set_text(format!("{} {}", charm_skill.get_skill().name, charm_skill.get_level()).as_str());
			self.skill[i].show();
		}
	}
}