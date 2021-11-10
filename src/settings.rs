use std::io::Write;
use std::cell::RefCell;
use std::ops::Not;
use std::rc::Rc;
use std::fs::File;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use ron::ser::{PrettyConfig};
use crate::data::db::DB;

const CONFIG_FILE: &str = "mhwiss.conf";

#[derive(Serialize, Deserialize)]
pub struct Settings {
	#[serde(skip_serializing, skip_deserializing)]
	available_languages: Rc<Vec<(String, String)>> ,
	language: RefCell<String>,
}

impl Settings {
	pub fn new(db: &DB) -> Self {
		let available_languages = Rc::new(db.get_available_languages());
		let mut ret = Settings {
			available_languages: Rc::clone(&available_languages),
			language: RefCell::new(String::from("en"))
		};

		if let Some(proj_dirs) = ProjectDirs::from("org", "SilverCorp", "mhwiss") {
			let conf = proj_dirs.config_dir();
			let conf = conf.join(CONFIG_FILE);
			if conf.exists() {  // TODO add atomic file reader
				let file = File::open(conf).unwrap();
				let mut settings: Settings = ron::de::from_reader(file).expect("Failed to load config");
				settings.available_languages = available_languages;
				return settings;
			} else {
				println!("Use default config");
			}
		}
		ret
	}

	pub fn change_language(&self, lang: String) {
		self.language.replace(lang);
	}

	pub fn get_language(&self) -> String {
		self.language.borrow().clone()
	}

	pub fn get_available_languages(&self) -> Rc<Vec<(String, String)>> {
		Rc::clone(&self.available_languages)
	}

	pub fn write(&self) -> std::io::Result<()> {
		if let Some(proj_dirs) = ProjectDirs::from("org", "SilverCorp", "mhwiss") {
			let conf = proj_dirs.config_dir();
			if conf.exists().not() {
				std::fs::create_dir_all(conf)?;
			}
			let conf = conf.join(CONFIG_FILE);

			let mut file = File::create(conf)?;
			let pretty = PrettyConfig::new()
				.with_depth_limit(4)
				.with_indentor("\t".to_owned());
			let s = ron::ser::to_string_pretty(&self, pretty).expect("Serialization failed");
			file.write(s.as_bytes())?;
		}
		Ok(())
	}
}