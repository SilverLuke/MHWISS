use std::{
    cell::RefCell,
    collections::HashMap,
    fmt,
    ops::Not,
    sync::Arc,
};
use std::cell::Cell;
use std::thread::Builder;

use glib::Sender;
use strum::{Display, EnumIter, EnumString};

use crate::datatypes::{
    *,
    equipment::Equipment,
    forge::Forge,
};
use crate::engines::{
    genetic::Genetic,
    greedy::Greedy,
};
use crate::ui::Callback;

pub(crate) mod greedy;
pub(crate) mod genetic;

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

	pub fn run(&self, engine_type: Engines) -> Result<(), &str>{
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
				}).unwrap();
			} else {
				return Err("Add gui info, engine already running");
			}
		} else {
			return Err("Add gui info, no constrains");
		}
		Ok(())
	}

	pub fn add_callback(&self, sender: Sender<Callback>) {
		self.sender.replace(Some(sender));
	}

	pub fn ended(&self) {
		self.running.replace(false);
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
