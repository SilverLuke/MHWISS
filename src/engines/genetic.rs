use std::{
	sync::Arc,
	collections::HashMap
};
use crate::datatypes::{
	ID, Level,
	equipment::Equipment,
	forge::Forge,
};
use crate::engines::{
	Engine,
	EnginesManager
};

pub(crate) struct Genetic {
}

impl Genetic {
	pub(crate) fn new(forge: Arc<Forge>, constrains: HashMap<ID, Level>) -> Self {
		Genetic {

		}
	}
}

impl Engine for Genetic {
	fn run(&mut self) -> Vec<Equipment> {
		vec![Equipment::new()]
	}
}
