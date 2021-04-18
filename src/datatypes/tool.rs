use std::{
	fmt,
	sync::Arc,
	cell::RefCell,
	collections::{HashMap, hash_map::Entry},
};

use crate::datatypes::{
	ID, Level,
	types::Item,
	skill::{Skill, SkillLevel, SkillsLevel},
};

pub struct Tool {
	id: ID,
	pub name: String,
	slots: [u8; 2],
}

impl Tool {
	fn new(id:ID, name:String) -> Self {
		Tool {
			id: 1,
			name: String::from("Test"),
			slots: [1, 4],
		}
	}
}

impl Item for Tool {
	fn has_skills(&self, query: &HashMap<u16, u8>) -> bool {
		false
	}

	fn get_skills_chained(&self, chained: &mut HashMap<u16, u8>) {
		todo!()
	}

	fn get_skills_hash(&self) -> HashMap<u16, u8> {
		todo!()
	}

	fn get_skills_iter(&self) -> Box<dyn Iterator<Item=&SkillLevel>> {
		todo!()
	}

	fn get_slots(&self) -> Option<Vec<u8>> {
		Some(Vec::from(self.slots))
	}
}

impl fmt::Display for Tool {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.name)
	}
}