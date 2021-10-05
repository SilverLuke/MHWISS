use std::{
	sync::Arc,
};

use crate::data::{
	mutable::equipment::Equipment,
	db_storage::Storage,
	db_types::skill::SkillsLevel,
};
use crate::engines::{Engine, EngineError};
#[allow(dead_code)]
pub(crate) struct HillClimbing {
	storage: Arc<Storage>,
	constraints: SkillsLevel,

	start_points: Vec<Equipment>,
	iterations: u32,
	random: u32,
}

impl HillClimbing {
	pub(crate) fn new(storage: Arc<Storage>, constraints: SkillsLevel) -> Self {
		HillClimbing {
			storage: Arc::clone(&storage),
			constraints: constraints.clone(),
			start_points: vec![],
			iterations: 256,
			random: 0
		}
	}
}

impl Engine for HillClimbing {
	fn run(&mut self) -> Result<Vec<Equipment>, EngineError> {
		unimplemented!()
	}
}


