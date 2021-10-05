use std::{
	mem,
	sync::{Arc, Once},
	ops::Range,
};
use glib::random_int_range;
use rand::prelude::*;
use crate::data::{
	db_storage::Storage,
	db::{DB, get_skill_by_id},
	db_types::{
		ID, Skills,
		skill::{Skill, SkillLevel, SkillsLevel}
	},
};

const RANDOM_LEN: usize = 10;

mod datatype;
mod greedy;
mod genetic;


struct Shared {
	storage: Arc<Storage>,
	static_constraints : Arc<Vec<SkillsLevel>>,
	random_constraints : Arc<Vec<SkillsLevel>>,
}

fn static_constraints(storage: &Storage) -> Vec<SkillsLevel> {
	let mut static_constraints = Vec::new();

	let mut constraints = SkillsLevel::new();
	constraints.insert(SkillLevel::new(storage.get_skill_from_name("Occhio critico").unwrap(), 4));
	constraints.insert(SkillLevel::new(storage.get_skill_from_name("Artigiano").unwrap(), 3));
	static_constraints.push( constraints);

	let mut constraints = SkillsLevel::new();
	constraints.insert(SkillLevel::new(storage.get_skill_from_name("Occhio critico").unwrap(), 7));
	constraints.insert(SkillLevel::new(storage.get_skill_from_name("Bonus attacco").unwrap(), 7));
	constraints.insert(SkillLevel::new(storage.get_skill_from_name("Bonus critico").unwrap(), 3));
	constraints.insert(SkillLevel::new(storage.get_skill_from_name("Artigiano").unwrap(), 5));
	static_constraints.push( constraints);
	static_constraints
}

impl Shared {
	fn new() -> Arc<Self> {
		let mut storage = Storage::new();
		let db = DB::new("it".to_string());
		storage.load_all(&db);
		storage.print_stat();

		let mut rand = Vec::new();
		for _ in 0..RANDOM_LEN {
			if let Some(c) = generate_random_constraints(&storage.skills, random_int_range(1,10) as usize) {
				rand.push(c);
			}
		}
		let static_constraints = static_constraints(&storage);
		Arc::new(Shared {
			storage: Arc::new(storage),
			static_constraints: Arc::new(static_constraints),
			random_constraints: Arc::new(rand),
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

fn generate_random_constraints(skills: &Skills, len: usize) -> Option<SkillsLevel> {
	let mut constraints = SkillsLevel::new();
	let mut len = len;
	if skills.len() < len {
		return None;
	}
	let mut rng = rand::thread_rng();
	while len > 0 {
		let skill_id : ID = rng.gen_range::<ID, Range<ID>>(1..skills.len() as ID);
		if constraints.contains_id(skill_id) {
			continue;
		}
		if let Some(skill) = get_skill_by_id(skills, skill_id) {
			let range = 1..=skill.max_level;
			let level = rng.gen_range(range);
			constraints.insert(SkillLevel::new(Arc::clone(skill), level));
		} else {
			continue;
		}
		len -= 1;
	}
	Some(constraints)
}