use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use gdk_pixbuf::Pixbuf;
use gtk::Builder;
use gtk::prelude::{BuilderExtManual, ImageExt, LabelExt};

use crate::data::db_types::decoration::Decoration;

pub struct GtkSlot {
	pub(crate) label: gtk::Label,
	image: gtk::Image,
	images: Rc<HashMap<String, Pixbuf>>,
}

impl GtkSlot {
	pub fn new(builder: &Builder, label: &str, image: &str, images: Rc<HashMap<String, Pixbuf>>) -> Self {
		GtkSlot {
			label: builder.object(label).expect(label),
			image: builder.object(image).expect(image),
			images,
		}
	}

	pub fn update(&self, piece: &Option<Arc<Decoration>>, size: u8) {
		if let Some(deco) = piece {
			self.show(deco, size);
		} else {
			self.empty(size);
		}
	}

	pub fn empty(&self, size: u8) {
		self.label.set_text("");
		if size == 0 {
			self.image.set_from_pixbuf(self.images.get("slot none"));
		} else {
			self.image.set_from_pixbuf(self.images.get(format!("slot {} 0", size).as_str()));
		}
	}

	pub fn show(&self, item: &Arc<Decoration>, size: u8) {
		self.label.set_text(item.name.as_str());
		self.image.set_from_pixbuf(self.images.get(format!("slot {} {}", size, item.size).as_str()));
	}
}