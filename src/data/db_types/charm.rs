use std::{
    fmt,
    sync::Arc,
	hash::{Hash, Hasher},
};
use crate::data::db_types::{
	ID,
	Item,
	skill::{Skill, SkillLevel, SkillsLevel},
};


pub struct Charm {
	pub id: ID,
	pub name: String,
	pub skills: SkillsLevel,
}

impl Charm {
	pub fn new(id: ID, name: String) -> Self {
		Charm { id, name, skills: SkillsLevel::new() }
	}
	pub fn add_skill(&mut self, skill: &Arc<Skill>, level: u8) {
		self.skills.insert(SkillLevel::new( Arc::clone(skill), level));
	}
}

impl Item for Charm {
	fn get_skills(&self) -> SkillsLevel {
		self.skills.clone()
	}

	fn get_slots(&self) -> Vec<u8> {
		vec![]
	}
}

impl PartialEq for Charm {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}
impl Eq for Charm {}

impl Hash for Charm {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.id.hash(state);
	}
}

impl fmt::Display for Charm {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{0: <45}| {1: <45}", format!("{} [{}]", self.name, self.id), self.skills.to_string())
	}
}