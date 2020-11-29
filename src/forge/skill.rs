use std::rc::Rc;
use std::fmt;
use std::collections::HashMap;
use crate::forge::types::{ID, SkillsLev};

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
		write!(f, "{}[{}]", self.name, self.id)
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

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Charm {
	pub id: ID,
	pub name: String,
	pub skills: Vec<(Rc<Skill>, u8)>,
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

	pub fn get_skills_rank(&self, query: &HashMap<Rc<Skill>, u8>) -> Option<u8> {
		let mut rank: u8 = 0;
		for (skill, lev) in self.skills.iter() {
			if query.get(skill).is_some() {
				rank += lev;
			}
		}
		if rank == 0 {
			return None;
		}
		Some(rank)
	}
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Decoration {
	pub id: ID,
	pub name: String,
	pub size: u8,
	pub skills: SkillsLev,
}

impl fmt::Display for Decoration {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut str = String::new();
		for (skill, lev) in self.skills.iter() {
			str = format!("{} <{}, {}>", str, *skill, lev);
		}
		write!(f, "{}[{}] Skill: {}", self.name, self.id, str)
	}
}

impl Decoration {
	pub fn new(id: ID, name: String, size: u8, skills: Vec<(Rc<Skill>, u8)>) -> Self {
		Decoration { id, name, size, skills }
	}

	pub fn get_skills_rank(&self, query: &HashMap<Rc<Skill>, u8>) -> Option<u8> {
		let mut rank: u8 = 0;
		for (skill, lev) in self.skills.iter() {
			if query.get(skill).is_some() {
				rank += lev;
			}
		}
		if rank == 0 {
			return None;
		}
		Some(rank)
	}
}
