use std::{
	env,
	rc::Rc,
	str::FromStr,
};
use gio::prelude::*;
use glib::Receiver;
use gtk::{Application, ComboBoxText, prelude::*};
use strum::IntoEnumIterator;
use crate::ui::pages::Pages;
use crate::engines::{Engines, EnginesManager, EnginesManagerError};
use crate::settings::Settings;
use crate::data::{
	mutable::equipment::Equipment,
	db_storage::Storage,
};
use crate::data::dyn_storage::DynamicStorage;

pub(crate) mod pages;
pub(crate) mod items;

pub enum Callback {
	Done(Vec<Equipment>),
	Impossible,
}

pub struct Ui {
	application: gtk::Application,
	window: gtk::ApplicationWindow,
	find_btn: gtk::Button,
	lang_combo: gtk::ComboBoxText,
	engines_combo: gtk::ComboBoxText,

	notebook: gtk::Notebook,
	pages: Pages,

	settings: Settings,

	pub(crate) storage: Rc<Storage>,
	pub(crate) dynamic_storage: Rc<DynamicStorage>,
	pub(crate) engine_manager: Rc<EnginesManager>,
}

pub fn get_builder(path_from_root: String) -> gtk::Builder {
	let root_dir = {
		let exe = env::current_exe().unwrap();
		let dir = exe.parent().expect("Executable must be in some directory");
		let dir = dir.parent().unwrap().parent().unwrap().to_path_buf();
		dir
	};
	let glade = root_dir.join(path_from_root);
	gtk::Builder::from_file(glade)  // ToDo: Use new_from_resources with some cargo tricks
}

impl Ui {
	pub fn new(settings: Settings, storage: Storage) -> Rc<Self> {
		gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));
		let builder = get_builder("res/gui/main.glade".to_string());

		let application = Application::new(
			Some("mhwi.ss"),
			Default::default()
		);

		let window = builder.object("main window").unwrap();
		let find_btn = builder.object("find btn").unwrap();
		let lang_combo: ComboBoxText = builder.object("languages combo").unwrap();
		let engines_combo: ComboBoxText = builder.object("engines combo").unwrap();

		let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

		let engine_manager = Rc::new(EnginesManager::new(sender));

		let dynamic_storage =  Rc::new(DynamicStorage::new(&storage));
		let pages = Pages::new(&builder, &dynamic_storage);

		{ // Populate the combo box in the title bar
			for (i, val) in Engines::iter().enumerate() {
				engines_combo.insert(i as i32, Some(val.to_string().as_str()), val.to_string().as_str());
			}
			engines_combo.set_active_id(Some(Engines::Greedy.to_string().as_str()));

			for (i, (id, name)) in settings.get_available_languages().iter().enumerate() {
				lang_combo.insert(i as i32, Some(id), name.as_str())
			}
			lang_combo.set_active_id(Some(settings.get_language().as_ref()));
		}


		let app = Rc::new(Self {
			application,
			window,
			find_btn,
			lang_combo,
			engines_combo,

			notebook: builder.object("notebook").unwrap(),
			pages,
			settings,

			storage: Rc::new(storage),
			dynamic_storage,
			engine_manager,
		});
		app.connect_signals(receiver);
		app
	}

	fn connect_signals(self: &Rc<Self>, receiver: Receiver<Callback>) {
		// Button for starting the background engine thread
		{
			let app = Rc::clone(self);
			self.find_btn.connect_clicked(move |_btn| {
				let engine = Engines::from_str(app.engines_combo.active_text().unwrap().as_str()).unwrap();
				let result = app.engine_manager.spawn(engine, &app.dynamic_storage);
				if let Err(e) = result {
					match e {
						EnginesManagerError::AlreadyRunning => { println!("UI: Engine already running")}
						EnginesManagerError::NoConstraints => { println!("UI: No constraints") }
					}
				}
			});
		}
		// Language selector and change the language in the settings
		{
			let app = Rc::clone(self);
			self.lang_combo.connect_changed(move |new_lang| {
				let language = new_lang.active_id().unwrap();
				app.settings.change_language(language.parse().unwrap());
			});
		}
		// Callback for the UI update when the running engine return the best equipment.
		{
			let app = Rc::clone(self);
			receiver.attach(None, move |action| {
				match action {
					Callback::Done(results) => {
						app.engine_manager.ended();
						app.pages.found_page.update(results);
						app.notebook.set_current_page(Some(app.notebook.n_pages() - 1));
					}
					Callback::Impossible => {
						println!("Engine: Impossible to find");
						// TODO add gui message
					}
				}
				glib::Continue(true)
			});
		}
		// Some signal TODO is this usefull??
		{
			let window = self.window.clone();
			self.application.connect_activate(move |app| {
				app.add_window(&window);
				window.present();
			});
		}
		// On shutdown -> update config file
		{
			let app = Rc::clone(self);
			self.application.connect_shutdown(move |_me| {
				let err = app.settings.write();
				if err.is_err() {
					println!("{:?}", err);
				}
			});
		}
	}

	pub fn start(self: &Rc<Self>) {
		self.pages.insert_widgets_tabs(Rc::clone(self));
		self.window.show_all();
		self.application.run();
	}
}
