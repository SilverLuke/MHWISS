use std::rc::Rc;
use std::fmt;
use std::collections::HashMap;
use std::cell::RefCell;
use crate::datatypes::{ID, SkillsLev};

pub trait HasSkills {
	fn has_skills(&self, query: &RefCell<HashMap<ID, u8>>) -> bool;
	fn get_skills_rank(&self, query: &RefCell<HashMap<ID, u8>>) -> Option<u8>;
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Skill {
	pub id: ID,
	pub name: String,
	pub description: String,
	pub max_level: u8,
	pub secret: u8,
	pub unlock: Option<Rc<Skill>>,
}

impl fmt::Display for Skill {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} [{}]", self.name, self.id)
	}
}

impl Skill {
	pub fn new(id: ID, name: String, description: String, max_level: u8, secret: u8, unlock: Option<Rc<Skill>>) -> Self {
		Skill { id, name, description, max_level, secret, unlock }
	}
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SetSkill {
	pub id: ID,
	pub name: String,
	pub skills: SkillsLev,
}

impl SetSkill {
	pub fn new(id: ID, name: String) -> Self {
		SetSkill { id, name, skills: Vec::new() }
	}

	pub fn add_skill(&mut self, skill: &Rc<Skill>, lev: u8) {
		self.skills.push((Rc::clone(skill), lev));
	}

	pub fn get_max(&self) -> u8 {
		let mut max = 0;
		for i in self.skills.iter() {
			if i.1 > max {
				max = i.1;
			}
		}
		max
	}
}
