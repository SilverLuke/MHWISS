use gio::prelude::*;
use gtk::prelude::*;
use gtk::Application;
use std::{env,
	env::args,
	rc::Rc,
	sync::Arc,
	collections::HashMap
};
use itertools::Itertools;
use gdk_pixbuf::Pixbuf;

use super::pages::{
	skills::SkillsPage,
	armors::ArmorsPage,
	decorations::DecorationsPage,
	charms::CharmsPage,
	result::ResultPage
};
use crate::datatypes::forge::Forge;
use crate::searcher::searcher::Searcher;
use crate::datatypes::types::{Element, ArmorClass};
use crate::ui::{NORMAL_SIZE_ICON, SMALL_SIZE_ICON};

struct Pages {
	skills_page: SkillsPage,
	armors_page: ArmorsPage,
	decos_page: DecorationsPage,
	charms_page: CharmsPage,
	found_page: ResultPage,
}

impl Pages {
	pub fn new(builder: &gtk::Builder, images: Rc<HashMap<String, Pixbuf>>) -> Self {
		Pages {
			skills_page: SkillsPage::new(&builder),
			armors_page:ArmorsPage::new(&builder),
			decos_page: DecorationsPage::new(&builder),
			charms_page: CharmsPage::new(&builder),
			found_page: ResultPage::new(builder, images),
		}
	}

	pub fn show(&self, app: Rc<Ui>) {  // Todo move this to the costructor this methods do not show anything they load things
		let forge = Arc::clone(&app.forge);
		self.skills_page.show(app, &forge);
		self.armors_page.show(&forge);
		self.decos_page.show(&forge);
		self.charms_page.show(&forge);
	}
}

pub struct Ui {
	application: gtk::Application,
	window: gtk::ApplicationWindow,
	find_btn: gtk::Button,
	lang_combo: gtk::ComboBox,
	images: Rc<HashMap<String, Pixbuf>>,

	notebook: gtk::Notebook,
	pages: Pages,

	forge: Arc<Forge>,
	pub searcher: Searcher,
}

impl Ui {
	pub fn get_builder(path_from_root: String) -> gtk::Builder {
		let root_dir = {
			let exe = env::current_exe().unwrap();
			let dir = exe.parent().expect("Executable must be in some directory");
			let dir = dir.parent().unwrap().parent().unwrap().to_path_buf();
			dir
		};
		let glade = root_dir.join(path_from_root);
		gtk::Builder::from_file(glade)  // ToDo: Use new_from_resurces with some cargo tricks
	}

	pub fn new(forge: Arc<Forge>, searcher: Searcher) -> Rc<Self> {
		gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));
		let builder = Ui::get_builder("gui/main.glade".to_string());

		let application = Application::new(
			Some("mhwi.ass"),
			Default::default()
		).expect("Initialization failed...");

		let window = builder.get_object("main window").unwrap();
		let find_btn = builder.get_object("find btn").unwrap();
		let lang_combo = builder.get_object("lang combo").unwrap();

		let images = Rc::new(Ui::load_images());
		let pages = Pages::new(&builder, Rc::clone(&images));
		let app = Self {
			application,
			window,
			find_btn,
			lang_combo,
			images,

			notebook: builder.get_object("notebook").unwrap(),
			pages,
			forge,
			searcher,
		};
		let tmp = Rc::new(app);
		tmp.setup_signals(tmp.clone());
		tmp
	}

	fn setup_signals(&self, me: Rc<Self>) {
		let window = self.window.clone();
		self.application.connect_activate(move |app| {
			app.add_window(&window);
			window.present();
		});

		let app = Rc::clone(&me);
		self.find_btn.connect_clicked(move |_btn| {
			let res = app.searcher.calculate();
			app.searcher.show_requirements();
			println!("{}", &res);
			app.pages.found_page.update(&res);
			let i = app.notebook.get_n_pages() - 1;
			app.notebook.set_current_page(Some(i));
		});
	}

	pub fn start(&self, me: Rc<Self>) {
		self.pages.show(Rc::clone(&me));
		self.window.show_all();
		let args: Vec<String> = args().collect();
		self.application.run(&args);
	}

	pub fn load_images() -> HashMap<String, Pixbuf> {
		let mut resources = vec![
			(String::from("weapon empty"), String::from("equipment/weapon empty.svg"), NORMAL_SIZE_ICON),
			(String::from("charm"), String::from("equipment/charm.svg"), NORMAL_SIZE_ICON),
			(String::from("charm empty"), String::from("equipment/charm empty.svg"), NORMAL_SIZE_ICON),
			(String::from("mantle"), String::from("equipment/mantle.svg"), NORMAL_SIZE_ICON),
			(String::from("mantle empty"), String::from("equipment/mantle empty.svg"), NORMAL_SIZE_ICON),
			(String::from("booster"), String::from("equipment/booster.svg"), NORMAL_SIZE_ICON),
			(String::from("slot none"), String::from("ui/slot none.svg"), SMALL_SIZE_ICON),
		];
		// for i in Weapons::iterator(){}
		for i in Element::iterator() {
			let name = i.to_string();
			let path = format!("ui/{}.svg", &name);
			resources.push((name, path, SMALL_SIZE_ICON));
		}
		for i in ArmorClass::iterator() {
			let name = i.to_string();
			let path = format!("equipment/{}.svg", &name);
			resources.push((name.clone(), path, NORMAL_SIZE_ICON));
			let res_name = format!("{} empty", &name);
			let path = format!("equipment/{} empty.svg", &name);
			resources.push((res_name, path, NORMAL_SIZE_ICON));
		}

		for i in 1..=4 {
			for j in 0..=i {  // slot <slot size> <deco size>
				resources.push((format!("slot {} {}", i, j), format!("ui/slot {} {}.svg", i, j), SMALL_SIZE_ICON));
			}
		}

		let mut hash: HashMap<String, Pixbuf> = Default::default();
		resources.into_iter().for_each(|(name, path, size)| {
			let real_path = format!("MHWorldData/images/{}", path);
			hash.insert(name, Pixbuf::from_file_at_scale(&real_path, size, size, true).expect(&path));
		});
		hash
	}

	pub(crate) fn set_image(image: &gtk::Image, key: &str, images: &Rc<HashMap<String, Pixbuf>>) {
		let pixbuf = images.get(key);
		assert_ne!(pixbuf, None);
		image.set_from_pixbuf(pixbuf);
	}

	pub fn set_fixed_image(builder: &gtk::Builder, id: &str, path: &str, size: i32) {
		let path = format!("MHWorldData/images/{}", path);
		let image: gtk::Image = builder.get_object(id).expect(id);
		image.set_from_pixbuf(
			Some(&Pixbuf::from_file_at_scale(&path, size, size, true).expect(path.as_str()))
		);
	}

}
