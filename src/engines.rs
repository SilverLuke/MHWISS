mod greedy;
mod genetic;

use std::{
	fmt,
	rc::Rc,
	ops::Not,
	sync::Arc,
	cell::RefCell,
	borrow::Borrow,
	cmp::Ordering,
	collections::{
		hash_map::Entry,
		HashMap,
	},
};
use itertools::Itertools;
use glib::Sender;

use crate::datatypes::{
	*,
	equipment::Equipment,
	armor::Armor,
	charm::Charm,
	decoration::{Decoration, AttachedDecorations},
	forge::Forge,
};
use crate::ui::ui::Callback;

use crate::engines::{
	greedy::Greedy,
	genetic::Genetic,
};

enum Engines {
	Greedy,
	Genetic,
}

trait Engine {
	fn new(forge: Arc<Forge>, constrains: HashMap<ID, Level>) -> Self where Self: Sized;

	fn run(&self) -> Equipment;
}

pub struct EnginesManager {
	pub(crate) forge: Arc<Forge>,
	skills_constraints: RefCell<HashMap<ID, Level>>,
	sender: Option<Sender<Callback>>,
}

impl EnginesManager {
	pub fn new(forge: Arc<Forge>) -> Self {
		let searcher = EnginesManager {
			forge,
			skills_constraints: Default::default(),
			sender: None,
		};
		searcher
	}

	pub fn add_constraint(&self, skill_id: ID, lev: u8) {
		if lev == 0 {
			self.skills_constraints.borrow_mut().remove(&skill_id);
		} else {
			self.skills_constraints.borrow_mut().insert(skill_id, lev);
		}
	}

	pub fn run(&self) {
		let forge = Arc::clone(&self.forge);
		let constrains = self.skills_constraints.borrow().clone();
		let engine_type = Engines::Greedy;
		let sender = self.sender.clone();
		std::thread::spawn(move || {
			let engine =
				match engine_type {
					Engines::Greedy  => Box::new(Greedy::new(forge, constrains)) as Box<dyn Engine>,
					Engines::Genetic => Box::new(Genetic::new(forge, constrains)) as Box<dyn Engine>,
				};
			let best_equipment = engine.run();

			println!("{}", &best_equipment);
			if let Some(sender) = sender {
				sender.send(Callback::Done(best_equipment)).unwrap();
			}
		});
	}

	pub fn add_callback(&mut self, sender: Sender<Callback>) {
		self.sender = Some(sender);
	}
}

impl fmt::Display for EnginesManager {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut str = String::new();
		for (id, lev) in self.skills_constraints.borrow().iter() {
			str = format!("{} <{}, {}>", str, &self.forge.skills.get(id).unwrap().name, lev);
		}
		write!(f, "{}", str)
	}
}

#[cfg(test)]
#[path = "engines/tests.rs"]
mod tests;