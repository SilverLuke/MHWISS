use std::{
	fmt,
	sync::Arc,
	cell::RefCell,
	collections::HashMap,
};
use crate::datatypes::{
	ID, Level,
	skill::{Skill, SkillLevel, SkillsLevel},
	armor::Armor,
	types::Item,
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
		self.skills.update_or_append(SkillLevel::new( Arc::clone(skill), level));
	}
}

impl Item for Charm {
	fn has_skills(&self, query: &HashMap<u16, u8>) -> bool {
		self.skills.contains_hash(query)
	}

	fn get_skills_chained(&self, chained: &mut HashMap<u16, u8>) {
		self.skills.put_in(chained);
	}

	fn get_skills_hash(&self) -> HashMap<ID, Level> {
		self.skills.as_hash()
	}

	fn get_skills_iter(&self) -> Box<dyn Iterator<Item=&SkillLevel> + '_> {
		self.skills.get_skills()
	}

	fn get_slots(&self) -> Option<Vec<u8>> {
		None
	}
}

impl PartialEq for Charm {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}