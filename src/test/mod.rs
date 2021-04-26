use std::{
	borrow::Borrow,
	mem,
	ops::Not,
	rc::Rc,
	sync::{Arc, Mutex, Once},
	thread,
	thread::sleep,
	time::Duration,
	collections::HashMap,
};
use crate::datatypes::{
	forge::Forge,
	equipment::Equipment,
	ID, Level, MAX_SLOTS,
	types::{ArmorClass, Gender, ArmorRank, Item},
	skill::{Skill, SkillLevel, SkillsLevel},
	decoration::AttachedDecorations,
};
use crate::engines::{
	EnginesManager,
	greedy::Greedy,
	genetic::Genetic,
	Engine
};
use crate::db::DB;

mod greedy;
mod genetic;
mod datatype;

struct Shared {
	forge: Arc<Forge>,
}

impl Shared {
	fn new() -> Arc<Self> {
		let mut forge = Forge::new();
		let db = DB::new();
		db.set_lang("it".to_string());
		forge.load_all(db);
		forge.print_stat();
		Arc::new(Shared {
			forge: Arc::new(forge),
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
