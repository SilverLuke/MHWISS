use directories::ProjectDirs;
use std::fs::File;

use serde::{Deserialize, Serialize};
use ron::ser::{PrettyConfig};
use std::io::Write;
use std::cell::RefCell;
use std::ops::Not;

const CONFIG_FILE : &str = "mhwiss.conf";

#[derive(Serialize, Deserialize)]
pub struct Settings {
	language: RefCell<String>,
}

impl Settings {
	pub fn new() -> Self {
		if let Some(proj_dirs) = ProjectDirs::from("org", "SilverCorp",  "mhwiss") {
			let conf = proj_dirs.config_dir();
			let conf = conf.join(CONFIG_FILE);
			if conf.exists() {  // TODO add atomic file reader
				let file = File::open(conf).unwrap();
				match ron::de::from_reader(file) {
					Ok(x) => return x,
					Err(e) => {
						println!("Failed to load config: {}", e);
					}
				};
			}
		}
		println!("Use default config");
		Settings {
			language: RefCell::new(String::from("en")),
		}
	}

	pub fn change_language(&self, lang: String) {
		self.language.replace(lang);
	}

	pub fn get_language(&self) -> String {
		self.language.borrow().clone()
	}

	pub fn write(&self) -> std::io::Result<()> {
		if let Some(proj_dirs) = ProjectDirs::from("org", "SilverCorp",  "mhwiss") {
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