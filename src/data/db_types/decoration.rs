use std::{
    fmt,
	hash::{Hash, Hasher},
};
use crate::data::db_types::{
	ID, Item,
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

impl Item for Decoration {
	fn get_skills(&self) -> SkillsLevel {
		self.skills.clone()
	}
	fn get_slots(&self) -> Vec<u8> {
		vec![]
	}
}

impl PartialEq for Decoration {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}
impl Eq for Decoration {}

impl Hash for Decoration {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.id.hash(state);
	}
}

impl fmt::Display for Decoration {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{0: <45}|{1: <45}", format!("{} [{}]", self.name, self.id), self.skills.to_string())
	}
}

