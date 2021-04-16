use std::{
	fmt,
	sync::Arc,
	cell::RefCell,
	collections::HashMap,
};
use crate::datatypes::{
	ID,
	skill::{Skill, HasSkills, SkillLevel, SkillsLevel},
	armor::Armor
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

impl HasSkills for Charm {
	fn has_skills(&self, query: &HashMap<ID, u8>) -> bool {
		for skill in self.skills.get_skills() {
			if query.get(&skill.get_id()).is_some() {
				return true;
			}
		}
		false
	}

	fn get_skills_rank(&self, query: &HashMap<ID, u8>) -> u8 {
		let mut sum = 0;
		for skill in self.skills.get_skills() {
			if query.get(&skill.get_id()).is_some() {
				sum += skill.level;
			}
		}
		sum
	}
}

