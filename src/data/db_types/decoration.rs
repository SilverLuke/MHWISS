use std::{
    fmt,
};

use crate::data::db_types::{
	ID, HasSkills,
	skill::SkillsLevel,
};


pub struct Decoration {
	pub id: ID,
	pub name: String,
	pub size: u8,
	pub skills: SkillsLevel,
}

impl Decoration {
	pub fn new(id: ID, name: String, size: u8, skills: SkillsLevel) -> Self {
		Decoration { id, name, size, skills: skills }
	}
}

impl HasSkills for Decoration {
	fn get_skills(&self) -> SkillsLevel {
		self.skills.clone()
	}
}

impl fmt::Display for Decoration {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut str = String::new();
		for skill_level in self.skills.iter() {
			str = format!("{} <{}, {}>", str, *skill_level.get_skill(), skill_level.get_level());
		}
		write!(f, "{0: <45}|{1: <50}", format!("{} [{}]", self.name, self.id), str)
	}
}

impl PartialEq for Decoration {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}
