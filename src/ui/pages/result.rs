use std::{
	collections::HashMap,
	rc::Rc,
};
use gdk_pixbuf::Pixbuf;
use gtk::prelude::*;
use itertools::Itertools;
use strum::IntoEnumIterator;
use crate::data::{
	db_types::{Item, ArmorClass, Element},
	mutable::equipment::Equipment,
};
use crate::ui::{
	items::{
		armor::GtkArmour,
		charm::GtkCharm,
		tool::GtkTool,
		UI,
		weapon::GtkWeapon,
	},
	pages::{
		SMALL_SIZE_ICON,
		set_fixed_image,
		set_image
	},
	get_builder,
};

pub struct ResultPage {
	weapon: GtkWeapon,
	armors: Vec<GtkArmour>,
	charm: GtkCharm,
	tools: [GtkTool; 2],
	results_list: gtk::ListBox,
	skills_summary: gtk::ListBox,
	decorations_summary: gtk::ListBox,
	defences_summary: Vec<gtk::Label>,
	images: Rc<HashMap<String, Pixbuf>>,
}

impl ResultPage {
	pub fn new(builder: &gtk::Builder, images: Rc<HashMap<String, Pixbuf>>) -> Self {
		let iter = ArmorClass::iter();
		let mut armors = Vec::with_capacity(iter.len());
		for piece in iter {
			armors.push(GtkArmour::new(&builder, piece, Rc::clone(&images)));
		}
		let mut defences_summary = Vec::with_capacity(Element::iter_element().len() + 1);
		defences_summary.push(builder.object("total defence").unwrap());
		for ele in Element::iter_element() {
			let msg = format!("total {}", ele.to_string());
			defences_summary.push(builder.object(&msg).expect(&msg));
		}
		let f = ResultPage {
			weapon: GtkWeapon::new(builder, Rc::clone(&images)),
			armors,
			charm: GtkCharm::new(builder, Rc::clone(&images)),
			tools: [GtkTool::new(builder, 0, Rc::clone(&images)), GtkTool::new(builder, 1, Rc::clone(&images))],
			results_list: builder.object("results list").unwrap(),
			skills_summary: builder.object("skills summary").unwrap(),
			decorations_summary: builder.object("decorations summary").unwrap(),
			defences_summary,
			images,
		};
		f.set_fixed_images(builder);
		f.empty();
		f
	}

	fn set_fixed_images(&self, builder: &gtk::Builder) {
		set_fixed_image(builder, "weapon affinity image", "ui/affinity.svg", SMALL_SIZE_ICON);
		set_fixed_image(builder, "weapon attack image", "ui/attack.svg", SMALL_SIZE_ICON);

		for piece in ArmorClass::iter() {
			let i = piece as usize;
			set_fixed_image(builder, &format!("defense image {}", i), "ui/defense.svg", SMALL_SIZE_ICON);
			for element in Element::iter_element() {
				let img: gtk::Image = builder.object(&format!("{} image {}", element.to_string(), i)).expect(element.to_string().as_str());
				set_image(&img, element.to_string().as_str(), &self.images);
			}
		}

		set_fixed_image(builder, "total defense image", "ui/defense.svg", SMALL_SIZE_ICON);
		for element in Element::iter_element() {
			let img: gtk::Image = builder.object(&format!("total {} image", element.to_string())).expect(element.to_string().as_str());
			set_image(&img, element.to_string().as_str(), &self.images);
		}
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
		self.defences_summary.get(0).unwrap().set_text("");
		self.defences_summary.get(1).unwrap().set_text("");
		self.defences_summary.get(2).unwrap().set_text("");
		self.defences_summary.get(3).unwrap().set_text("");
		self.defences_summary.get(4).unwrap().set_text("");
		self.defences_summary.get(5).unwrap().set_text("");
	}

	pub fn update(&self, best_list: Vec<Equipment>) {
		let best = best_list.first().unwrap();
		self.weapon.update(&best.weapon);
		for (i, piece) in self.armors.iter().enumerate() {
			piece.update(&best.set[i]);
		}
		self.charm.update(&best.charm);
		for (i, tool) in self.tools.iter().enumerate() {  // Tools
			tool.update(&best.tools[i]);
		}
		for (i, _equip) in best_list.iter().enumerate() {
			let label = gtk::LabelBuilder::new().build();
			label.set_text(format!("{}", i).as_str());
			let child = gtk::ListBoxRowBuilder::new().build();
			child.add(&label);
			self.results_list.add(&child);
		}

		// Populate the skills summary ListBox
		self.skills_summary.forall(|i| { self.skills_summary.remove(i) });
		for skill_level in best.get_skills().iter().sorted_by(|a, b| { b.cmp(&a) }) {  // Skills Summary
			let builder = get_builder("res/gui/summary row.glade".to_string());
			let name: gtk::Label = builder.object("skill name").unwrap();
			name.set_text(format!("{} {}", skill_level.get_skill().name, skill_level.get_level()).as_str());
			let row: gtk::ListBoxRow = builder.object("skill row").unwrap();
			self.skills_summary.add(&row);
		}
		// Populate the decorations summary ListBox
		self.decorations_summary.forall(|i| { self.decorations_summary.remove(i) });
		for (decoration, quantity) in best.get_used_decorations().iter().sorted_by(|(_, quantiy), (_, q)| { q.cmp(&quantiy) }) {  // Skills Summary
			let builder = get_builder("res/gui/summary row.glade".to_string());
			let image: gtk::Image = builder.object("decoration image").unwrap();
			set_image(&image, format!("slot {} {}", decoration.size, decoration.size).as_str(), &self.images);
			let name: gtk::Label = builder.object("decoration name").unwrap();
			name.set_text(decoration.name.as_str());
			let quantity_label: gtk::Label = builder.object("decoration quantity").unwrap();
			quantity_label.set_text(format!("x{}", quantity).as_str());
			let row: gtk::ListBoxRow = builder.object("decoration row").unwrap();
			self.decorations_summary.add(&row);
		}
		// Final statistics
		self.defences_summary.get(0).unwrap().set_text(best.get_defence().to_string().as_str());
		self.defences_summary.get(1).unwrap().set_text(best.get_fire_defence().to_string().as_str());
		self.defences_summary.get(2).unwrap().set_text(best.get_water_defence().to_string().as_str());
		self.defences_summary.get(3).unwrap().set_text(best.get_thunder_defence().to_string().as_str());
		self.defences_summary.get(4).unwrap().set_text(best.get_ice_defence().to_string().as_str());
		self.defences_summary.get(5).unwrap().set_text(best.get_dragon_defence().to_string().as_str());
	}
}
