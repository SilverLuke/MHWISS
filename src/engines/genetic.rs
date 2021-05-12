#![allow(unused_variables)]
use std::{
    collections::HashMap,
    sync::Arc
};

use crate::datatypes::{
    equipment::Equipment, forge::Forge,
    ID,
    Level,
};
use crate::engines::Engine;

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
