use std::{
    cell::{RefCell, Cell},
    fmt,
    ops::Not,
    sync::Arc,
	thread::Builder,
};
use glib::Sender;
use strum::{Display, EnumIter, EnumString};
use crate::data::{
    mutable::equipment::Equipment,
	db_storage::Storage,
	db_types::skill::{SkillsLevel, SkillLevel},
};
use crate::engines::{
    hill_climbing::HillClimbing,
    greedy::Greedy,
};
use crate::ui::Callback;

pub(crate) mod greedy;
pub(crate) mod hill_climbing;

#[derive(Display, EnumString, EnumIter)]
pub enum Engines {
	Greedy,
	HillClimbing,
}

pub(crate) trait Engine {
	fn run(&mut self) -> Vec<Equipment>;
}

pub struct EnginesManager {
	storage: Arc<Storage>,
	constraints: RefCell<SkillsLevel>,
	sender: RefCell<Option<Sender<Callback>>>,
	running: Cell<bool>,
}

impl EnginesManager {
	pub fn new(storage: Arc<Storage>) -> Self {
		let searcher = EnginesManager {
			storage,
			constraints: RefCell::new(SkillsLevel::new()),
			sender: RefCell::new(None),
			running: Cell::new(false),
		};
		searcher
	}

	pub fn add_constraint(&self, skill: SkillLevel) {
		self.constraints.borrow_mut().insert(skill);
	}

	pub fn clean_constrains(&self) {
		self.constraints.replace(SkillsLevel::new());
	}

	pub fn run(&self, engine_type: Engines) -> Result<(), &str>{
		if self.constraints.borrow().len() > 0 {
			if self.running.get().not() {
				self.running.replace(true);
				let storage = Arc::clone(&self.storage);
				let constraints = RefCell::clone(&self.constraints);
				let sender = self.sender.try_borrow().unwrap().clone();
				//self.sender.replace(sender.clone());
				println!("Constrains: {}", constraints.borrow());
				Builder::new().name(engine_type.to_string().into()).spawn(move || {
					let mut engine = match engine_type {
						Engines::Greedy => Box::new(Greedy::new(storage, constraints.into_inner())) as Box<dyn Engine>,
						Engines::HillClimbing => Box::new(HillClimbing::new(storage, constraints.into_inner())) as Box<dyn Engine>,
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
			return Err("Add gui info, no constraints");
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
		for skill_level in self.constraints.borrow().iter() {
			str = format!("{} <{}, {}>", str, skill_level.get_skill().name, skill_level.get_level());
		}
		write!(f, "{}", str)
	}
}
