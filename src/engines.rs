use std::{
	cell::{RefCell, Cell},
	fmt,
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
	EnginesManagerError::{AlreadyRunning, NoConstraints},
};
use crate::ui::Callback;

pub(crate) mod greedy;
pub(crate) mod hill_climbing;

#[derive(Display, EnumString, EnumIter)]
pub enum Engines {
	Greedy,
	HillClimbing,
}

pub enum EngineError {
	Impossible,
}

pub enum EnginesManagerError {
	AlreadyRunning,
	NoConstraints,
}


pub(crate) trait Engine {
	fn run(&mut self) -> Result<Vec<Equipment>, EngineError>;
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
		self.constraints.borrow_mut().set(skill);
	}

	pub fn clean_constrains(&self) {
		self.constraints.replace(SkillsLevel::new());
	}

	pub fn spawn(&self, engine_type: Engines) -> Result<(), EnginesManagerError> {
		if self.constraints.borrow().len() <= 0 {
			return Err(NoConstraints);
		}
		if self.running.get() {
			return Err(AlreadyRunning);
		}

		self.running.replace(true);
		let storage = Arc::clone(&self.storage);
		let constraints = RefCell::clone(&self.constraints);
		let sender = self.sender.try_borrow().unwrap().clone();
		println!("Constrains: {}", constraints.borrow());

		Builder::new().name(engine_type.to_string().into()).spawn(move || {
			let mut engine = match engine_type {
				Engines::Greedy => Box::new(Greedy::new(storage, constraints.into_inner())) as Box<dyn Engine>,
				Engines::HillClimbing => Box::new(HillClimbing::new(storage, constraints.into_inner())) as Box<dyn Engine>,
			};
			let best_equipment = engine.run();

			if let Some(sender) = sender {
				match best_equipment {
					Ok(bests) => {
						println!("{}", bests.first().unwrap());
						sender.send(Callback::Done(bests)).expect("Error sending callback");
					}
					Err(e) => match e {
						EngineError::Impossible => {
							sender.send(Callback::Impossible).expect("Error sending callback");
						}
					}
				}
			} else {
				println!("No ui callback found, this should never happen! X_X");
			}
		}).unwrap();
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
