use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::fmt;
use crate::datatypes::{ID, SkillsLev};
use crate::datatypes::skill::{Skill, HasSkills};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Charm {
	pub id: ID,
	pub name: String,
	pub skills: SkillsLev,
}

impl fmt::Display for Charm {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut str = String::new();
		for (skill, lev) in self.skills.iter() {
			str = format!("{} <{}, {}>", str, *skill, lev);
		}
		write!(f, "{}[{}] Skill: {}", self.name, self.id, str)
	}
}

impl Charm {
	pub fn new(id: ID, name: String) -> Self {
		Charm { id, name, skills: Vec::with_capacity(1) }
	}
	pub fn add_skill(&mut self, skill: &Rc<Skill>, level: u8) {
		self.skills.push((Rc::clone(skill), level));
	}
}

impl HasSkills for Charm {
	fn has_skills(&self, query: &RefCell<HashMap<ID, u8>>) -> bool {
		for (skill, _lev) in self.skills.iter() {
			if query.borrow().get(&skill.id).is_some() {
				return true;
			}
		}
		false
	}

	fn get_skills_rank(&self, query: &RefCell<HashMap<ID, u8>>) -> Option<u8> {
		let mut sum = 0;
		for (skill, lev) in self.skills.iter() {
			if query.borrow().get(&skill.id).is_some() {
				sum += lev;
			}
		}
		if sum != 0 {
			return Some(sum);
		}
		None
	}
}

