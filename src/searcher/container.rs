use std::{
	cell::RefCell,
	rc::Rc,
	collections::HashMap,
};
use crate::forge::{
	skill::{Skill, Decoration},
	types::{ID, Level, SkillLev}
};
use std::collections::hash_map::Entry;

pub trait HasDecorations {
	fn get_slots(&self) -> [u8;3];
	fn get_skills(&self) -> Box<dyn Iterator<Item=&SkillLev> + '_>;
}

pub trait HasSkills {
	fn has_skills(&self, query: &RefCell<HashMap<ID, u8>>) -> bool;
}

pub struct DecorationContainer<T: HasDecorations + HasSkills> {
	pub container: Rc<T>,
	deco: [Option<Rc<Decoration>>; 3],
	pub value: u8,
}

impl<T> Clone for DecorationContainer<T> where T: HasDecorations + HasSkills {
	fn clone(&self) -> Self {
		DecorationContainer {
			container: Rc::clone(&self.container),
			deco: self.deco.clone(),
			value: self.value,
		}
	}
}

impl<T> DecorationContainer<T> where T: HasDecorations + HasSkills{
	pub fn new(container: Rc<T>) -> Self {
		DecorationContainer {
			container,
			deco: [None, None, None],
			value: 0
		}
	}

	pub fn value(&mut self, req: &HashMap<ID, u8>) -> u8 {
		let mut value = 0;
		for (skill, lev) in self.container.get_skills() {
			if req.contains_key(&skill.id) {
				value += lev;
			}
		}
		/*
		for slot in armor.slots {
			for (deco, val) in self.decorations.borrow().iter() {
			}
		}
		*/
		self.value = value;
		value
	}

	pub fn get_container(&self) -> &Rc<T> {
		&self.container
	}

	fn is_empty(&self, i: usize) -> bool {
		self.deco[i].is_none()
	}

	pub fn try_add_deco(&mut self, deco: &Rc<Decoration>) -> Result<(), &str> {
		for (i, size) in self.container.get_slots().iter().enumerate().rev() {
			if *size >= deco.size {
				if self.is_empty(i) {
					self.deco[i] = Some(Rc::clone(deco));
					return Ok(());
				}
			}
		}
		Err("No space left")
	}

	pub fn add_skills(&self, skills_sum: &mut HashMap<ID, Level>) {
		for (skill, lev) in self.container.get_skills() {
			match skills_sum.entry(skill.id) {
				Entry::Occupied(mut o) => o.insert(o.get() + lev),
				Entry::Vacant(v) => *v.insert(*lev)
			};
		}
		for deco in self.deco.iter() {
			if let Some(deco) = deco {
				for (skill, lev) in deco.skills.iter() {
					match skills_sum.entry(skill.id) {
						Entry::Occupied(mut o) => o.insert(o.get() + lev),
						Entry::Vacant(v) => *v.insert(*lev)
					};
				}
			}
		}
	}

	pub fn get_skills(&self) -> HashMap<ID, Level> {
		let mut skills: HashMap<ID, Level> = Default::default();
		for (skill, lev) in self.container.get_skills() {
			match skills.entry(skill.id) {
				Entry::Occupied(mut o) => o.insert(o.get() + lev),
				Entry::Vacant(v) => *v.insert(*lev)
			};
		}
		for deco in self.deco.iter() {
			if let Some(deco) = deco {
				for (skill, lev) in deco.skills.iter() {
					match skills.entry(skill.id) {
						Entry::Occupied(mut o) => o.insert(o.get() + lev),
						Entry::Vacant(v) => *v.insert(*lev)
					};
				}
			}
		}
		skills
	}
}