use std::collections::HashMap;
use std::rc::Rc;

use gdk_pixbuf::Pixbuf;
use gtk::{Builder, ImageExt, LabelExt};
use gtk::prelude::BuilderExtManual;

use crate::datatypes::decoration::AttachedDecorations;
use crate::datatypes::tool::Tool;
use crate::ui::items::slots::GtkSlot;
use crate::ui::items::UI;

pub struct GtkTool {
	name: gtk::Label,
	pub image: gtk::Image,
	slots: Vec<GtkSlot>,
	images: Rc<HashMap<String, Pixbuf>>,
}

impl GtkTool {
	pub fn new(builder: &Builder, id: u8, images: Rc<HashMap<String, Pixbuf>>) -> Self {
		let mut slots = Vec::with_capacity(2);
		for i in 0..2 {
			let name = format!("tool slot {} {}", i, id);
			let image = format!("tool slot image {} {}", i, id);
			slots.push(GtkSlot::new(&builder, name.as_str(), image.as_str(), Rc::clone(&images)));
		}
		GtkTool {
			name: builder.get_object(format!("tool name {}", id).as_str()).unwrap(),
			image: builder.get_object(format!("tool image {}", id).as_str()).unwrap(),
			slots,
			images
		}
	}
}

impl UI<AttachedDecorations<Tool>> for GtkTool {
	fn update(&self, piece: &Option<AttachedDecorations<Tool>>) {
		if let Some(tool) = piece {
			self.show(tool);
		} else {
			self.empty();
		}
	}

	fn empty(&self) {
		self.image.set_from_pixbuf(self.images.get("mantle empty"));
		self.name.set_text("-");
		self.slots[0].empty(0);
		self.slots[1].empty(0);
	}

	fn show(&self, item: &AttachedDecorations<Tool>) {
		let tool = item.get_item();
		self.image.set_from_pixbuf(self.images.get("booster empty"));
		self.name.set_text(tool.name.as_str());
	}
}