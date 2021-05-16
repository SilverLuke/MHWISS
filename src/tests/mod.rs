mod datatype;
mod greedy;
mod genetic;

use std::{
	mem,
	sync::{Arc, Once},
	collections::HashMap,
};

use crate::datatypes::{
	forge::Forge,
	ID, Level,
};
use crate::db::DB;

struct Shared {
	forge: Arc<Forge>,
	constrains : Arc<HashMap<ID, Level>>
}

impl Shared {
	fn new() -> Arc<Self> {
		let mut forge = Forge::new();
		let db = DB::new();
		db.set_lang("it".to_string());
		forge.load_all(&db);
		forge.print_stat();
		let mut constrains : HashMap<ID, Level> = Default::default();
		constrains.insert(forge.get_skill_from_name("Occhio critico").unwrap().id, 4);
		constrains.insert(forge.get_skill_from_name("Artigiano").unwrap().id, 3);
		Arc::new(Shared {
			forge: Arc::new(forge),
			constrains: Arc::new(constrains),
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
