use std::{
	rc::Rc,
	cell::Cell,
	thread::Builder,
};
use strum::{Display, EnumIter, EnumString};
use glib::Sender;
use crate::data::{
	mutable::equipment::Equipment,
	dyn_storage::DynamicStorage,
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
	sender: Option<Sender<Callback>>,
	running: Cell<bool>,
}

impl EnginesManager {
	pub fn new(sender: Sender<Callback>) -> Self {
		let searcher = EnginesManager {
			sender: Some(sender),
			running: Cell::new(false),
		};
		searcher
	}

	pub fn spawn(&self, engine_type: Engines, dynamic: &Rc<DynamicStorage>) -> Result<(), EnginesManagerError> {
		let storage = dynamic.generate_storage();
		let constraints = dynamic.get_constraints();
		if constraints.len() <= 0 {
			return Err(NoConstraints);
		}
		if self.running.get() {
			return Err(AlreadyRunning);
		}

		self.running.replace(true);


		let sender = self.sender.clone();
		println!("Constrains: {}", &constraints);

		Builder::new().name(engine_type.to_string().into()).spawn(move || {
			let mut engine = match engine_type {
				Engines::Greedy => Box::new(Greedy::new(storage, constraints)) as Box<dyn Engine>,
				Engines::HillClimbing => Box::new(HillClimbing::new(storage, constraints)) as Box<dyn Engine>,
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

	pub fn ended(&self) {
		self.running.replace(false);
	}
}
