use std::{
	fmt,
	sync::Arc,
	cell::RefCell,
	collections::{HashMap, hash_map::Entry},
};

use crate::datatypes::{
	ID, Level,
	skill::{Skill, HasSkills, SkillLevel, SkillsLevel},
	decoration::HasDecorations
};
use std::fmt::Formatter;

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
			slots: [1,4],
		}
	}
}

impl HasDecorations for Tool {
	fn get_slots(&self) -> [u8; 3] {
		todo!()
	}

	fn get_skills(&self) -> Box<dyn Iterator<Item=&SkillLevel>> {
		todo!()
	}
}

impl HasSkills for Tool {
	fn has_skills(&self, query: &HashMap<ID, Level>) -> bool {
		todo!()
	}

	fn get_skills_rank(&self, query: &HashMap<ID, Level>) -> u8 {
		todo!()
	}
}

impl fmt::Display for Tool {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.name)
	}
}