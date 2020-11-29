use gtk::prelude::*;
use gdk_pixbuf::{Pixbuf};
use gtk::Builder;
use crate::forge::types::{Element, WeaponClass};
use crate::forge::types::ArmorClass;
use std::collections::HashMap;
use crate::forge::searcher::{BestSet, ArmorDeco};
use crate::forge::weapon::Weapon;
use crate::forge::armor::Armor;
use std::rc::Rc;

const NORMAL_SIZE: i32 = 60;
const SMALL_SIZE: i32 = 25;

struct GtkSlot {
	image: gtk::Image,
	label: gtk::Label,
}

impl GtkSlot {
	fn new(builder: &Builder, name: &str, image: &str) -> Self {
		GtkSlot {
			image: builder.get_object(image).unwrap(),
			label: builder.get_object(name).unwrap()
		}
	}
}

struct GtkWeapon {
	name: gtk::Label,
	image: gtk::Image,
	attack: gtk::Label,
	affinity: gtk::Label,
	element: [gtk::Label; 2],
	skill: gtk::Label,
	slots: Vec<GtkSlot>,
}

impl GtkWeapon {
	fn new(builder: &Builder) -> Self {
		let mut slots = Vec::with_capacity(3);
		for i in 0..3 {
			let name = format!("weapon slot {}", i);
			let image = format!("weapon slot image {}", i);
			slots.push(GtkSlot::new(&builder, name.as_str(), image.as_str()));
		}
		GtkWeapon {
			name: builder.get_object("weapon name").unwrap(),
			image: builder.get_object("weapon image").unwrap(),
			attack: builder.get_object("weapon attack").unwrap(),
			affinity: builder.get_object("weapon affinity").unwrap(),
			element: [builder.get_object("weapon element 1").unwrap(), builder.get_object("weapon element 2").unwrap()],
			skill: builder.get_object("weapon skill").unwrap(),
			slots
		}
	}

	pub fn update(&self, weapon: &Option<Rc<Weapon>>, images: &HashMap<String, Pixbuf>) {
		if weapon.is_none() {
			self.image.set_from_pixbuf(images.get("weapon empty"));
			self.name.set_text("-");
			self.attack.set_text("-");
			self.affinity.set_text("-");
			self.skill.set_text("-");
		}
	}
}

struct GtkArmour {
	name: gtk::Label,
	image: gtk::Image,
	class: ArmorClass,
	defence: gtk::Label,
	elements: Vec<gtk::Label>,
	// Size 5
	skill: [gtk::Label; 2],
	slots: Vec<GtkSlot>,
}

impl GtkArmour {
	fn new(builder: &Builder, piece: ArmorClass) -> Self {
		let piece_id = piece as u8;
		let iter = Element::iter_element();
		let mut elements = Vec::with_capacity(iter.len());
		for ele in iter {
			elements.push(builder.get_object(format!("{} {}", ele.to_string(), piece_id).as_str()).unwrap());
		}

		let mut slots = Vec::with_capacity(3);
		for i in 0..3 {
			let name = format!("slot {} {}", i, piece_id);
			let image = format!("slot image {} {}", i, piece_id);
			slots.push(GtkSlot::new(&builder, name.as_str(), image.as_str()));
		}
		GtkArmour {
			name: builder.get_object(&format!("name {}", piece_id)).unwrap(),
			image: builder.get_object(&format!("image {}", piece_id)).unwrap(),
			class: piece,
			defence: builder.get_object(&format!("defence {}", piece_id)).unwrap(),
			elements,
			skill: [builder.get_object(&format!("skill 0 {}", piece_id)).unwrap(), builder.get_object(&format!("skill 1 {}", piece_id)).unwrap()],
			slots,
		}
	}

	pub fn update(&self, piece: &Option<ArmorDeco>, images: &HashMap<String, Pixbuf>) {
		if let Some(piece) = piece {
			let piece = piece.get_armor();
			self.image.set_from_pixbuf(images.get(format!("{}", self.class.to_string()).as_str()));
			self.name.set_text(piece.name.as_str());
			for (i, (skill, lev)) in piece.skills.iter().enumerate() {
				self.skill[i].set_text(format!("{} {}", skill.name, lev).as_str());
				self.skill[i].show();
			}
			for (i, size) in piece.slots.iter().enumerate() {
				let str = {
					if *size != 0 {
						format!("slot {} 0", size)
					} else {
						String::from("slot none")
					}
				};
				let img = images.get(str.as_str());
				assert_ne!(img, None, "Image not found: {}", str);
				self.slots[i].image.set_from_pixbuf(img);
				self.slots[i].image.show();
				self.slots[i].label.set_text("-");
			}
		} else {
			self.image.set_from_pixbuf(images.get(format!("{} empty", self.class.to_string()).as_str()));
			self.name.set_text("-");
			self.skill[0].hide();
			self.skill[1].hide();
		}
	}
}

struct GtkCharm {
	name: gtk::Label,
	skill: [gtk::Label; 2],
}

struct GtkTool {
	name: gtk::Label,
	slots: Vec<GtkSlot>,
}

pub struct Found {
	weapon: GtkWeapon,
	armors: Vec<GtkArmour>,
	// Size 5
	// charm: Charm,
	// tools: [Tool; 2],
	list: gtk::ListBox,
	images: HashMap<String, Pixbuf>,
}

impl Found {
	pub fn new(builder: &gtk::Builder) -> Found {
		let images = Found::load_images();
		let iter = ArmorClass::iterator();
		let mut armors = Vec::with_capacity(iter.len());
		for piece in iter {
			armors.push(GtkArmour::new(&builder, *piece));
		}
		let f = Found {
			weapon: GtkWeapon::new(builder),
			armors,
			list: builder.get_object("results list").unwrap(),
			images,
		};
		f.set_images(builder);
		f.set_weapon();
		f
	}

	fn load_images() -> HashMap<String, Pixbuf> {
		let mut resources = vec![
			(String::from("weapon empty"), String::from("equipment/weapon empty.svg"), NORMAL_SIZE),
			(String::from("charm"), String::from("equipment/charm.svg"), NORMAL_SIZE),
			(String::from("charm empty"), String::from("equipment/charm empty.svg"), NORMAL_SIZE),
			(String::from("mantle"), String::from("equipment/mantle.svg"), NORMAL_SIZE),
			(String::from("mantle empty"), String::from("equipment/mantle empty.svg"), NORMAL_SIZE),
			(String::from("booster"), String::from("equipment/booster.svg"), NORMAL_SIZE),
			(String::from("slot none"), String::from("ui/slot none.svg"), SMALL_SIZE),
		];
		// for i in Weapons::iterator(){}
		for i in Element::iterator() {
			let name = i.to_string();
			let path = format!("ui/{}.svg", &name);
			resources.push((name, path, SMALL_SIZE));
		}
		for i in ArmorClass::iterator() {
			let name = i.to_string();
			let path = format!("equipment/{}.svg", &name);
			resources.push((name.clone(), path, NORMAL_SIZE));
			let res_name = format!("{} empty", &name);
			let path = format!("equipment/{} empty.svg", &name);
			resources.push((res_name, path, NORMAL_SIZE));
		}

		for i in 1..=4 {
			for j in 0..=i {
				resources.push((format!("slot {} {}", i, j), format!("ui/slot {} {}.svg", i, j), SMALL_SIZE));
			}
		}

		let mut hash: HashMap<String, Pixbuf> = Default::default();
		resources.into_iter().for_each(|(name, path, size)| {
			let real_path = format!("MHWorldData/images/{}", path);
			hash.insert(name, Pixbuf::from_file_at_scale(&real_path, size, size, true).expect(&path));
		});
		hash
	}

	pub fn set_weapon(&self) {
		let i = Pixbuf::from_file_at_scale("MHWorldData/images/equipment/weapon empty.svg", NORMAL_SIZE, NORMAL_SIZE, false).expect("Can't get pixbuf");
		self.weapon.image.set_from_pixbuf(Some(&i));
	}

	fn set_image(&self, image: &gtk::Image, key: &str) {
		let pixbuf = self.images.get(key);
		assert_ne!(pixbuf, None);
		image.set_from_pixbuf(pixbuf);
	}

	fn set_static_image(&self, builder: &gtk::Builder, id: &str, path: &str, size: i32) {
		let path = format!("MHWorldData/images/{}", path);
		let image: gtk::Image = builder.get_object(id).expect(id);
		image.set_from_pixbuf(
			Some(&Pixbuf::from_file_at_scale(&path, size, size, true).expect(path.as_str()))
		);
	}

	fn set_images(&self, builder: &gtk::Builder) {
		self.set_static_image(builder, "weapon affinity image", "ui/affinity.svg", SMALL_SIZE);
		self.set_static_image(builder, "weapon attack image", "ui/attack.svg", SMALL_SIZE);

		for piece in ArmorClass::iterator() {
			let i = *piece as usize;
			let armor = &self.armors[i];
			self.set_image(&(armor.image), &format!("{} empty", piece.to_string()));
			self.set_static_image(builder, &format!("defense image {}", i), "ui/defense.svg", SMALL_SIZE);
			for element in Element::iter_element() {
				let img: gtk::Image = builder.get_object(&format!("{} image {}", element.to_string(), i)).expect(element.to_string().as_str());
				self.set_image(&img, element.to_string().as_str());
			}
		}

		self.set_static_image(builder, "charm image", "equipment/charm empty.svg", NORMAL_SIZE);
		self.set_static_image(builder, "tool image 1", "equipment/mantle empty.svg", NORMAL_SIZE);
		self.set_static_image(builder, "tool image 2", "equipment/booster empty.svg", NORMAL_SIZE);
	}

	pub fn update(&self, best: &BestSet) {
		self.weapon.update(&best.weapon, &self.images);
		for (i, piece) in self.armors.iter().enumerate() {
			piece.update(&best.set[i], &self.images);
		}
	}
}
