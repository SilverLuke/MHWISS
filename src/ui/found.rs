use gtk::prelude::*;
use gdk_pixbuf::{Pixbuf};

const IMAGE_SIZE: i32 = 50;
const SMALL_SIZE: i32 = IMAGE_SIZE / 2;

struct Weapon {
	name: gtk::Label,
	image: gtk::Image,
	attack: gtk::Label,
	affinity: gtk::Label,
	element: [gtk::Label; 2],
	skill: gtk::Label,
	slots: [gtk::Label; 3],
}

struct Armours {
	name: gtk::Label,
	defence: gtk::Label,
	element: [gtk::Label; 5],
	skill: [gtk::Label; 2],
	slots: [gtk::Label; 3],
}

struct Charm {
	name: gtk::Label,
	skill: [gtk::Label; 2],
}

struct Tool {
	name: gtk::Label,
	slots: [gtk::Label; 2],
}

pub struct Found {
	weapon: Weapon,
	// armours: [Armours; 5],
	// charm: Charm,
	// tools: [Tool; 2],
	// list: gtk::ListBox
}

impl Found {
	pub fn new(builder: &gtk::Builder) -> Found {
		let weapon = Weapon{
			name: builder.get_object("weapon name").unwrap(),
			image: builder.get_object("weapon image").unwrap(),
			attack: builder.get_object("weapon attack").unwrap(),
			affinity: builder.get_object("weapon affinity").unwrap(),
			element: [builder.get_object("weapon element 1").unwrap(), builder.get_object("weapon element 2").unwrap()],
			skill: builder.get_object("weapon skill").unwrap(),
			slots: [builder.get_object("weapon slot 1").unwrap(), builder.get_object("weapon slot 2").unwrap(), builder.get_object("weapon slot 3").unwrap()]
		};
		let f = Found {
			weapon
		};
		f.set_images(builder);
		f.set_weapon();

		f
	}

	pub fn set_weapon(&self) {
		let i = Pixbuf::from_file_at_scale("MHWorldData/images/equipment/ic_equipment_greatsword_base.svg", IMAGE_SIZE , IMAGE_SIZE, false).expect("Can't get pixbuf");
		self.weapon.image.set_from_pixbuf(Some(&i));
	}

	fn load(&self, builder: &gtk::Builder, id: &str, path: &str, size: i32) {
		let path: &str = &format!("{}{}", "MHWorldData/images/", path);
		let image: gtk::Image = builder.get_object(id).expect(id);
		image.set_from_pixbuf(
			Some(&Pixbuf::from_file_at_scale(path, size, size, true).expect(path))
		);
	}

	fn set_images(&self, builder: &gtk::Builder) {
		self.load(builder, "weapon affinity image", "ui/ic_ui_affinity.svg", SMALL_SIZE);
		self.load(builder, "weapon attack image", "ui/ic_ui_attack.svg", SMALL_SIZE);
		for i in 1..=5 {
			self.load(builder, &format!("defense image {}", i), "ui/ic_ui_defense.svg", SMALL_SIZE);

			for j in ["fire", "ice", "thunder", "water", "dragon"].iter() {
				self.load(builder, &format!("{} image {}", j, i), &format!("ui/ic_element_{}.svg", j), SMALL_SIZE);
			}
		}
	}

}