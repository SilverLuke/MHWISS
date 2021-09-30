mod datatype;
mod greedy;
mod genetic;

use std::{
	mem,
	sync::{Arc, Once},
};

use crate::data::{
	db_storage::Storage,
	db::DB,
	db_types::{
		skill::{Skill, SkillsLevel}
	},
};
use crate::data::db_types::skill::SkillLevel;

struct Shared {
	storage: Arc<Storage>,
	constraints : Arc<SkillsLevel>
}

impl Storage {
	pub fn get_skill_from_name(&self, name: &str) -> Option<Arc<Skill>> {
		for skill in self.skills.iter() {
			if skill.name == name {
				return Some(Arc::clone(skill));
			}
		}
		None
	}
}

impl Shared {
	fn new() -> Arc<Self> {
		let mut storage = Storage::new();
		let db = DB::new("it".to_string());
		storage.load_all(&db);
		storage.print_stat();
		let mut constraints = SkillsLevel::new();
		constraints.insert(SkillLevel::new(storage.get_skill_from_name("Occhio critico").unwrap(), 4));
		constraints.insert(SkillLevel::new(storage.get_skill_from_name("Artigiano").unwrap(), 3));
		Arc::new(Shared {
			storage: Arc::new(storage),
			constraints: Arc::new(constraints),
		})
	}

	pub fn get() -> Arc<Self> {
		static mut SINGLETON: Option<Arc<Shared>> = None;
		static ONCE: Once = Once::new();
		unsafe {
			ONCE.call_once(|| {
				let singleton = Shared::new();
				SINGLETON = Some(mem::transmute(singleton));
			});
			Arc::clone(SINGLETON.as_ref().unwrap())
		}
	}
}
