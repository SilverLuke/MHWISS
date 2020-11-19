use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application};
use std::env::args;
use std::env;
use std::rc::Rc;
use itertools::Itertools;

use crate::forge;
use crate::ui::found::Found;

pub struct Skills {
	skill_list: gtk::FlowBox,
	skill_set: gtk::FlowBox,
}


pub struct Ui {
	application: gtk::Application,
	window: gtk::ApplicationWindow,
	notebook: gtk::Notebook,
	skill_list: gtk::FlowBox,
	skill_set: gtk::FlowBox,
	rank_set: [gtk::ListBox; 3],
	deco_list: [gtk::FlowBox; 4],
	find_btn: gtk::Button,
	lang_combo: gtk::ComboBox,
	forge: Rc<forge::forge::Forge>,
	searcher: forge::searcher::Searcher,
	found: Found,
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

	pub fn new() -> Rc<Self> {
		gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));
		let builder = Ui::get_builder("gui/main.glade".to_string());

		let application = Application::new(
			Some("mhwi.ass"),
			Default::default()
		).expect("Initialization failed...");

		let window = builder.get_object("main window").unwrap();
		let find_btn = builder.get_object("find btn").unwrap();
		let skill_list = builder.get_object("skill list").unwrap();
		let skill_set = builder.get_object("skill set").unwrap();
		let lang_combo = builder.get_object("lang combo").unwrap();
		let rank_set = [
			builder.get_object("lr list").unwrap(),
			builder.get_object("hr list").unwrap(),
			builder.get_object("mr list").unwrap(),
		];
		let deco_list = [
			builder.get_object("deco lev1").unwrap(),
			builder.get_object("deco lev2").unwrap(),
			builder.get_object("deco lev3").unwrap(),
			builder.get_object("deco lev4").unwrap(),
		];

		let forge = Rc::new(forge::forge::Forge::new());
		let searcher = forge::searcher::Searcher::new(Rc::clone(&forge));
		let app = Self {
			application,
			window,
			notebook: builder.get_object("notebook").unwrap(),
			skill_list,
			skill_set,
			rank_set,
			deco_list,
			find_btn,
			lang_combo,
			forge,
			searcher,
			found: Found::new(&builder),
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
			app.found.update(&res);
			let i = app.notebook.get_n_pages() - 1;
			app.notebook.set_current_page(Some(i));
		});
	}

	fn show_skills(&self, me: Rc<Self>) {  // TODO: Add skill dependecy and separe skill from set skills
		for skill in self.forge.skills.borrow().values().sorted_by(|a, b| {a.name.cmp(&b.name)}) {
			let builder = Ui::get_builder("gui/skill box.glade".to_string());
			let skill_box: gtk::Box = builder.get_object("box").unwrap();
			let name: gtk::Label = builder.get_object("name").unwrap();
			let adjustment: gtk::Adjustment = builder.get_object("adjustment").unwrap();
			let level: gtk::SpinButton = builder.get_object("level").unwrap();

			let style = skill_box.get_style_context();
			let provider = gtk::CssProvider::new();
			provider.load_from_path("gui/style.css").unwrap();
			style.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
			style.add_class("skillBox");  // TODO: Better implementation using glades => Add this feature in glade

			name.set_text(skill.name.as_str());
			name.set_tooltip_text(Some(skill.description.as_str()));
			adjustment.set_upper(skill.max_level as f64);

			let app = Rc::clone(&me);
			let skill_req = Rc::clone(&skill);
			level.connect_value_changed(move |lev| {
				app.searcher.add_skill_requirement(skill_req.clone(), lev.get_value() as u8);
			});

			if skill.id % 2 == 0 {
				self.skill_list.insert(&skill_box, -1);
			}else {
				self.skill_set.insert(&skill_box, -1);
			}
		}
	}

	fn show_sets(&self) {
		for set in self.forge.sets.borrow().values().sorted_by(|a, b| {a.id.cmp(&b.id)}) {
			let builder = Ui::get_builder("gui/set box.glade".to_string());
			let set_row: gtk::ListBoxRow = builder.get_object("row").unwrap();
			let name: gtk::Label = builder.get_object("name").unwrap();

			name.set_text(&set.name);
			self.rank_set[set.rank as usize].insert(&set_row, -1);
		}
	}

	fn show_deco(&self) {
		for (_, deco) in self.forge.decorations.borrow().iter().sorted_by(|(_, a), (_,b)| {a.name.cmp(&b.name)}) {
			let builder = Ui::get_builder("gui/deco box.glade".to_string());
			let deco_box: gtk::Box = builder.get_object("box").unwrap();
			let name: gtk::Label = builder.get_object("name").unwrap();

			let style = deco_box.get_style_context();
			let provider = gtk::CssProvider::new();
			provider.load_from_path("gui/style.css").unwrap();
			style.add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_USER);
			style.add_class("skillBox");  // TODO: Better implementation using glades => Add this feature in glade

			name.set_text(deco.name.as_str());
			self.deco_list[deco.size as usize - 1].insert(&deco_box, -1);
		}
	}

	fn show_all(&self, me: Rc<Self>) {
		self.show_skills(me);
		self.show_sets();
		self.show_deco();
	}

	pub fn start(&self, me: Rc<Self> ) {
		let lang = "it";
		self.forge.load_all(lang);
		self.show_all(me);
		self.window.show_all();
		let args: Vec<String> = args().collect();
		self.application.run(&args);
	}
}
