
use std::fmt;
use std::collections::HashMap;
use std::cell::RefCell;
use crate::datatypes::{ID, Level};
use std::sync::Arc;

pub trait HasSkills {
	fn has_skills(&self, query: &HashMap<ID, Level>) -> bool;
	fn get_skills_rank(&self, query: &HashMap<ID, Level>) -> u8;
}

pub struct Skill {
	pub id: ID,
	pub name: String,
	pub description: String,
	pub max_level: u8,
	pub secret: u8,
	pub unlock: Option<Arc<Skill>>,
}

impl Skill {
	pub fn new(id: ID, name: String, description: String, max_level: u8, secret: u8, unlock: Option<Arc<Skill>>) -> Self {
		Skill { id, name, description, max_level, secret, unlock }
	}
}

impl PartialEq for Skill {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}
impl Eq for Skill {}

impl fmt::Display for Skill {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} [{}]", self.name, self.id)
	}
}


pub struct SetSkill {
	pub id: ID,
	pub name: String,
	pub skills: SkillsLevel,
}

impl SetSkill {
	pub fn new(id: ID, name: String) -> Self {
		SetSkill { id, name, skills: SkillsLevel::new() }
	}

	pub fn add_skill(&mut self, skill: &Arc<Skill>, lev: u8) {
		self.skills.update_or_append(SkillLevel::new(Arc::clone(skill), lev));
	}

	// Get the max required set skill for enable the skill
	pub fn get_max(&self) -> u8 {
		let mut max= 0 ;
		for i in self.skills.get_skills() {
			if i.level > max {
				max = i.level;
			}
		}
		max
	}
}

impl PartialEq for SetSkill {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}
impl Eq for SetSkill {}


pub struct SkillLevel {
	pub(crate) skill: Arc<Skill>,
	pub(crate) level: Level
}

impl SkillLevel {
	pub fn new(skill: Arc<Skill>, level: Level) -> Self {
		SkillLevel { skill, level }
	}

	pub fn get_id(&self) -> ID {
		self.skill.id
	}
}

impl fmt::Display for SkillLevel {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} {}", self.skill.name, self.level)
	}
}

pub struct SkillsLevel {
	list: Vec<SkillLevel>
}

impl SkillsLevel {
	pub fn new() -> Self {
		SkillsLevel {
			list: Vec::new(),
		}
	}

	pub fn update_or_append(&mut self, add: SkillLevel) -> &Self {
		match self.list.iter_mut().find(|ref p| p.skill == add.skill) {
			Some(skill) => {  // If there is one, insert into it and update the level
				skill.level += add.level;
			}
			None => {  // Else push the new skill
				self.list.push(add);
			}
		}
		self
	}

	pub fn update_or_remove(&mut self, remove: SkillLevel) -> &Self {
		match self.list.iter_mut().find(|ref i| i.skill == remove.skill) {
			Some(skill) => {  // If there is one, insert into it and update the level
				if skill.level >= remove.level {
					skill.level -= remove.level;
				} else {
					self.list.retain(|ref i| i.skill != remove.skill);
				}
			}
			None => ()
		}
		self
	}

	pub(crate) fn get_skills(&self) -> Box<dyn Iterator<Item=&SkillLevel> + '_> {
		Box::new(self.list.iter())
	}

	pub(crate) fn shrink_to_fit(&mut self) {
		self.list.shrink_to_fit();
	}

	pub(crate) fn len(&self) -> usize {
		self.list.len()
	}
}

impl fmt::Display for SkillsLevel {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut str = String::new();
		for skill in self.list.iter() {
			str = format!("{} <{}>", str, skill);
		}
		write!(f, "{}", str)
	}
}
