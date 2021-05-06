pub(crate) mod greedy;
pub(crate) mod genetic;

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
	slice::Iter,
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
use strum::{EnumIter, Display, EnumString};
use std::thread::Builder;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;
use std::cell::Cell;

#[derive(Display, EnumString, EnumIter)]
pub enum Engines {
	Greedy,
	Genetic,
}

pub(crate) trait Engine {
	fn run(&mut self) -> Vec<Equipment>;
}

pub struct EnginesManager {
	forge: Arc<Forge>,
	skills_constraints: RefCell<HashMap<ID, Level>>,
	sender: RefCell<Option<Sender<Callback>>>,
	running: Cell<bool>,
}

impl EnginesManager {
	pub fn new(forge: Arc<Forge>) -> Self {
		let searcher = EnginesManager {
			forge,
			skills_constraints: Default::default(),
			sender: RefCell::new(None),
			running: Cell::new(false),
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

	pub fn clean_constrains(&self) {
		self.skills_constraints.replace(Default::default());
	}

	pub fn run(&self, engine_type: Engines) -> Result<(), ()>{
		if self.skills_constraints.borrow().len() > 0 {
			if self.running.get().not() {
				self.running.replace(true);
				let forge = Arc::clone(&self.forge);
				let constrains = self.skills_constraints.borrow().clone();
				let sender = self.sender.try_borrow().unwrap().clone();
				//self.sender.replace(sender.clone());
				println!("Constrains: {:?}", self.skills_constraints.borrow());
				Builder::new().name(engine_type.to_string().into()).spawn(move || {
					let mut engine = match engine_type {
						Engines::Greedy => Box::new(Greedy::new(forge, constrains)) as Box<dyn Engine>,
						Engines::Genetic => Box::new(Genetic::new(forge, constrains)) as Box<dyn Engine>,
					};
					let best_equipment = engine.run();

					if let Some(sender) = sender {
						let err = sender.send(Callback::Done(best_equipment));
						println!("{:?}", err);
					} else {
						println!("No ui callback");
					}
				});
			} else {
				println!("Add gui info, engine already running");
				return Err(());
			}
		} else {
			println!("Add gui info, no constrains");
			return Err(());
		}
		Ok(())
	}

	pub fn add_callback(&self, sender: Sender<Callback>) {
		self.sender.replace(Some(sender));
	}

	pub fn ended(&self) {
		println!("ENDED");
		self.running.replace(false);
	}

	pub fn get_constrains(&self) -> HashMap<ID, Level> {
		self.skills_constraints.borrow().clone()
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