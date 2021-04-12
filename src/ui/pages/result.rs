use std::collections::HashMap;
use std::rc::Rc;
use gdk_pixbuf::Pixbuf;
use gtk::Builder;
use gtk::prelude::*;

use crate::datatypes::{armor::Armor,
	weapon::Weapon,
	types::{Element, ArmorClass}
};
use crate::ui::{NORMAL_SIZE_ICON, SMALL_SIZE_ICON,
	items::{weapon::GtkWeapon,
		armor::GtkArmour,
		charm::GtkCharm,
		tool::GtkTool,
		UI,
	}
};
use crate::searcher::{bestset::BestSet};
use crate::ui::ui::Ui;
use std::ops::Index;

pub struct ResultPage {
	weapon: GtkWeapon,
	armors: Vec<GtkArmour>,
	charm: GtkCharm,
	tools: [GtkTool; 2],
	list: gtk::ListBox,
	images: Rc<HashMap<String, Pixbuf>>,
}

impl ResultPage {
	pub fn new(builder: &gtk::Builder, images: Rc<HashMap<String, Pixbuf>>) -> Self {
		let iter = ArmorClass::iterator();
		let mut armors = Vec::with_capacity(iter.len());
		for piece in iter {
			armors.push(GtkArmour::new(&builder, *piece, Rc::clone(&images)));
		}
		let f = ResultPage {
			weapon: GtkWeapon::new(builder, Rc::clone(&images)),
			armors,
			charm: GtkCharm::new(builder, Rc::clone(&images)),
			tools: [GtkTool::new(builder, 0, Rc::clone(&images)), GtkTool::new(builder, 1, Rc::clone(&images))],
			list: builder.get_object("results list").unwrap(),
			images,
		};
		f.set_fixed_images(builder);
		f.empty();
		f
	}

	fn empty(&self) {
		self.weapon.empty();
		for armor in self.armors.iter() {
			armor.empty();
		}
		self.charm.empty();
		for tool in self.tools.iter() {
			tool.empty();
		}
	}

	fn set_fixed_images(&self, builder: &gtk::Builder) {
		Ui::set_fixed_image(builder, "weapon affinity image", "ui/affinity.svg", SMALL_SIZE_ICON);
		Ui::set_fixed_image(builder, "weapon attack image", "ui/attack.svg", SMALL_SIZE_ICON);

		for piece in ArmorClass::iterator() {
			let i = *piece as usize;
			Ui::set_fixed_image(builder, &format!("defense image {}", i), "ui/defense.svg", SMALL_SIZE_ICON);
			for element in Element::iter_element() {
				let img: gtk::Image = builder.get_object(&format!("{} image {}", element.to_string(), i)).expect(element.to_string().as_str());
				Ui::set_image(&img, element.to_string().as_str(), &self.images);
			}
		}
	}

	pub fn update(&self, best: &BestSet) {
		self.weapon.update(&best.weapon);
		for (i, piece) in self.armors.iter().enumerate() {
			piece.update(&best.set[i]);
		}
		self.charm.update(&best.charm);
		for (i, tool) in self.tools.iter().enumerate() {
			tool.update(&best.tools[i]);
		}
	}
}
