use std::{
	env,
	env::args,
	rc::Rc,
	sync::Arc,
	collections::HashMap,
	str::FromStr,
};
use gtk::{prelude::*, Application, ComboBox, Label, ComboBoxText};
use glib::{Sender, Receiver};
use gio::prelude::*;
use gdk_pixbuf::Pixbuf;
use itertools::Itertools;
use strum::IntoEnumIterator;
use crate::ui::{
	NORMAL_SIZE_ICON, SMALL_SIZE_ICON,
	pages::{
		skills::SkillsPage,
		armors::ArmorsPage,
		decorations::DecorationsPage,
		charms::CharmsPage,
		result::ResultPage
	}
};
use crate::datatypes::{
	forge::Forge,
	types::{Element, ArmorClass},
	equipment::Equipment,
};
use crate::engines::{
	Engines,
	EnginesManager,
};
use gio::ListStore;
use crate::db::DB;

pub enum Callback {
	Done(Vec<Equipment>)
}

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
			armors_page: ArmorsPage::new(&builder),
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
	lang_combo: gtk::ComboBoxText,
	engines_combo: gtk::ComboBoxText,
	images: Rc<HashMap<String, Pixbuf>>,

	notebook: gtk::Notebook,
	pages: Pages,

	forge: Arc<Forge>,
	pub(crate) searcher: EnginesManager,
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

	pub fn new(forge: Arc<Forge>, mut searcher: EnginesManager) -> Rc<Self> {
		gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));
		let builder = Ui::get_builder("res/gui/main.glade".to_string());

		let application = Application::new(
			Some("mhwi.ss"),
			Default::default()
		).expect("Initialization failed...");

		let window = builder.get_object("main window").unwrap();
		let find_btn = builder.get_object("find btn").unwrap();
		let lang_combo:ComboBoxText = builder.get_object("languages combo").unwrap();
		let engines_combo:ComboBoxText = builder.get_object("engines combo").unwrap();
		let images = Rc::new(Ui::load_images());
		let pages = Pages::new(&builder, Rc::clone(&images));

		for (i, val) in Engines::iter().enumerate() {
			engines_combo.insert(i as i32, Some(val.to_string().as_str()), val.to_string().as_str());
		}
		engines_combo.set_active_id(Some(Engines::Greedy.to_string().as_str()));

		let db = DB::new();
		for (i, (id, name)) in db.load_languages().iter().enumerate() {
			lang_combo.insert(i as i32, Some(id), name.as_str())
		}
		lang_combo.set_active_id(Some("it"));

		let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
		searcher.add_callback(sender);

		let app = Rc::new(Self {
			application,
			window,
			find_btn,
			lang_combo,
			engines_combo,
			images,

			notebook: builder.get_object("notebook").unwrap(),
			pages,

			forge,
			searcher,
		});
		app.setup_signals(receiver);
		app
	}

	fn setup_signals(self: &Rc<Self>, receiver: Receiver<Callback>) {
		let window = self.window.clone();
		self.application.connect_activate(move |app| {
			app.add_window(&window);
			window.present();
		});

		let app = Rc::clone(self);
		self.find_btn.connect_clicked(move |_btn| {
			let engine = Engines::from_str(app.engines_combo.get_active_text().unwrap().as_str()).unwrap();
			app.searcher.run(engine);
		});

		let app= Rc::clone(self);
		receiver.attach(None, move |action| {
			app.process_callbacks(action)
		});
	}

	fn process_callbacks(self: &Rc<Self>, action: Callback) -> glib::Continue {
		match action {
			Callback::Done(results) => {
				self.pages.found_page.update(results);
				self.notebook.set_current_page(Some(self.notebook.get_n_pages() - 1));
				self.searcher.ended();
			}
		}
		glib::Continue(true)
	}

	pub fn start(self: &Rc<Self>) {
		self.pages.show(Rc::clone(self));
		self.window.show_all();
		let args: Vec<String> = args().collect();
		self.application.run(&args);
	}

	fn load_images() -> HashMap<String, Pixbuf> {
		let mut resources = vec![
			(String::from("weapon empty"), String::from("equipment/weapon empty.svg"), NORMAL_SIZE_ICON),
			(String::from("charm"), String::from("equipment/charm.svg"), NORMAL_SIZE_ICON),
			(String::from("charm empty"), String::from("equipment/charm empty.svg"), NORMAL_SIZE_ICON),
			(String::from("mantle"), String::from("equipment/mantle.svg"), NORMAL_SIZE_ICON),
			(String::from("mantle empty"), String::from("equipment/mantle empty.svg"), NORMAL_SIZE_ICON),
			(String::from("booster"), String::from("equipment/booster.svg"), NORMAL_SIZE_ICON),
			(String::from("slot none"), String::from("equipment/slot/none.svg"), SMALL_SIZE_ICON),
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
		let pixbuf = images.get(key);
		assert_ne!(pixbuf, None);
		image.set_from_pixbuf(pixbuf);
	}

	pub(crate) fn set_fixed_image(builder: &gtk::Builder, id: &str, path: &str, size: i32) {
		let path = format!("res/images/{}", path);
		let image: gtk::Image = builder.get_object(id).expect(id);
		image.set_from_pixbuf(
			Some(&Pixbuf::from_file_at_scale(&path, size, size, true).expect(path.as_str()))
		);
	}
}
