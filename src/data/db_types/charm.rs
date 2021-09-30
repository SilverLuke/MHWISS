use std::{
    fmt,
    sync::Arc,
};
use std::hash::{Hash, Hasher};

use crate::data::db_types::{
	ID,
	HasSkills,
	skill::{Skill, SkillLevel, SkillsLevel},
};

pub struct Charm {
	pub id: ID,
	pub name: String,
	pub skills: SkillsLevel,
}

impl fmt::Display for Charm {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}[{}] Skill: {}", self.name, self.id, self.skills)
	}
}

impl Charm {
	pub fn new(id: ID, name: String) -> Self {
		Charm { id, name, skills: SkillsLevel::new() }
	}
	pub fn add_skill(&mut self, skill: &Arc<Skill>, level: u8) {
		self.skills.insert(SkillLevel::new( Arc::clone(skill), level));
	}
}

impl HasSkills for Charm {
	fn get_skills(&self) -> SkillsLevel {
		self.skills.clone()
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