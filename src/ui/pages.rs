use std::{
	collections::HashMap,
	rc::Rc,
};
use gtk::{prelude::BuilderExtManual, prelude::ImageExt};
use gdk_pixbuf::{Pixbuf, InterpType};
use strum::IntoEnumIterator;
use crate::ui::{
	Ui,
	pages::{
		skills::SkillsPage,
		armors::ArmorsPage,
		decorations::DecorationsPage,
		charms::CharmsPage,
		result::ResultPage,
	}
};
use crate::data::{
	db_types::{ArmorClass, Element},
	dyn_storage::DynamicStorage,
};

pub mod skills;
pub mod armors;
pub mod charms;
pub mod decorations;
pub mod result;

pub const NORMAL_SIZE_ICON: i32 = 60;
pub const SMALL_SIZE_ICON: i32 = 25;

pub(crate) struct Pages {
	skills_page: SkillsPage,
	armors_page: Rc<ArmorsPage>,
	decos_page: DecorationsPage,
	charms_page: CharmsPage,
	pub(crate) found_page: ResultPage,
}

impl Pages {
	pub fn new(builder: &gtk::Builder, dynamic_storage: &Rc<DynamicStorage>) -> Self {

		let images = Rc::new(load_images());
		Pages {
			skills_page: SkillsPage::new(&builder, dynamic_storage),
			armors_page: Rc::new(ArmorsPage::new(&builder, Rc::clone(&images))),
			decos_page: DecorationsPage::new(&builder),
			charms_page: CharmsPage::new(&builder),
			found_page: ResultPage::new(builder, Rc::clone(&images)),
		}
	}

	// TOOD: This methods do not show anything instead this method create the UI widgets,
	// TOOD: we should implement some dynamic loading for better performance
	pub fn insert_widgets_tabs(&self, app: Rc<Ui>) {
		self.skills_page.show(&app.storage, &app.dynamic_storage);
		self.armors_page.show(&app.storage, &app.dynamic_storage);
		self.decos_page.show(&app.storage, &app.dynamic_storage);
		self.charms_page.show(&app.storage, &app.dynamic_storage);
	}
}

/* TODO how the hell i can return a closure ??
fn get_search_function(search_bar: &SearchEntry) -> Box<(Fn(& FlowBoxChild) -> bool)> {
	let search_bar = search_bar.clone();
	Box::new(move |child: &FlowBoxChild| {
		let gtkbox: gtk::Box = (child.child().unwrap()).downcast_ref::<gtk::Box>().unwrap().clone();
		for c in gtkbox.children() {
			if let Some(label) = c.downcast_ref::<gtk::Label>() {
				let deco_name = label.text();
				return deco_name.to_lowercase().contains(search_bar.text().to_lowercase().as_str());
			}
		}
		false
	})
}
*/

fn load_images() -> HashMap<String, Pixbuf> {
	let mut resources = vec![
		(String::from("weapon empty"), String::from("equipment/weapon empty.svg"), NORMAL_SIZE_ICON),
		(String::from("charm"), String::from("equipment/charm.svg"), NORMAL_SIZE_ICON),
		(String::from("charm empty"), String::from("equipment/charm empty.svg"), NORMAL_SIZE_ICON),
		(String::from("mantle"), String::from("equipment/mantle.svg"), NORMAL_SIZE_ICON),
		(String::from("mantle empty"), String::from("equipment/mantle empty.svg"), NORMAL_SIZE_ICON),
		(String::from("booster"), String::from("equipment/booster.svg"), NORMAL_SIZE_ICON),
		(String::from("slot none"), String::from("equipment/slot/none.svg"), SMALL_SIZE_ICON),
		(String::from("armor missing"), String::from("ui/armor missing.svg"), SMALL_SIZE_ICON),
	];
	// for i in Weapons::iterator(){}
	for i in Element::iter() {
		let name = i.to_string().to_lowercase();
		let path = format!("ui/{}.svg", &name);
		resources.push((name, path, SMALL_SIZE_ICON));
	}
	for i in ArmorClass::iter() {
		let name = i.to_string();
		let path = format!("equipment/{}.svg", &name);
		resources.push((name.clone(), path, NORMAL_SIZE_ICON));
		let res_name = format!("{} empty", &name);
		let path = format!("equipment/{} empty.svg", &name);
		resources.push((res_name, path, NORMAL_SIZE_ICON));
	}

	for i in 1..=4 {
		for j in 0..=i {  // slot <slot size> <deco size>
			resources.push((format!("slot {} {}", i, j), format!("equipment/slot/{} {}.svg", i, j), SMALL_SIZE_ICON));
		}
	}

	let mut hash: HashMap<String, Pixbuf> = Default::default();
	resources.into_iter().for_each(|(name, path, size)| {
		let real_path = format!("res/images/{}", path);
		hash.insert(name, Pixbuf::from_file_at_scale(&real_path, size, size, true).expect(&path));
	});
	hash
}

pub(crate) fn set_image(image: &gtk::Image, key: &str, images: &Rc<HashMap<String, Pixbuf>>) {
	let pixbuf = images.get(key).expect(&key);
	image.set_from_pixbuf(Some(pixbuf));
}

// TODO add in the images structure already scaled pixbuff -> refactoring images management
pub(crate) fn set_image_scaled(image: &gtk::Image, key: &str, size:i32, images: &Rc<HashMap<String, Pixbuf>>) {
	let pixbuf = images.get(key).expect(&key);
	let pixbuf = Pixbuf::scale_simple(pixbuf, size, size, InterpType::Nearest);
	image.set_from_pixbuf(Some(&pixbuf.unwrap()));
}



pub(crate) fn set_fixed_image(builder: &gtk::Builder, id: &str, path: &str, size: i32) {
	let path = format!("res/images/{}", path);
	let image: gtk::Image = builder.object(id).expect(id);
	image.set_from_pixbuf(
		Some(&Pixbuf::from_file_at_scale(&path, size, size, true).expect(path.as_str()))
	);
}
