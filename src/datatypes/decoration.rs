use std::{
	fmt,
	sync::Arc,
	cell::RefCell,
	collections::{HashMap, hash_map::Entry},
};
use crate::datatypes::{
	ID, Level, MAX_SLOTS,
	skill::{Skill, HasSkills, SkillLevel, SkillsLevel},
	armor::Armor
};

pub trait HasDecorations {
	fn get_slots(&self) -> [u8; 3];
	fn get_skills(&self) -> Box<dyn Iterator<Item=&SkillLevel> + '_>;
}

pub struct Decoration {
	pub id: ID,
	pub name: String,
	pub size: u8,
	pub skills: SkillsLevel,
}

impl fmt::Display for Decoration {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut str = String::new();
		for skill in self.skills.get_skills() {
			str = format!("{} <{}, {}>", str, *skill.skill, skill.level);
		}
		write!(f, "{0: <45}|{1: <50}", format!("{} [{}]", self.name, self.id), str)
	}
}

impl Decoration {
	pub fn new(id: ID, name: String, size: u8, skills: SkillsLevel) -> Self {
		Decoration { id, name, size, skills }
	}
}

impl HasSkills for Decoration {
	fn has_skills(&self, query: &HashMap<ID, Level>) -> bool {
		for skill in self.skills.get_skills() {
			if query.get(&skill.get_id()).is_some() {
				return true;
			}
		}
		false
	}

	fn get_skills_rank(&self, query: &HashMap<ID, Level>) -> u8 {
		let mut sum = 0;
		for skill in self.skills.get_skills() {
			if query.get(&skill.get_id()).is_some() {
				sum += skill.level;
			}
		}
		sum
	}
}


pub struct AttachedDecorations<T: HasDecorations + HasSkills> {
	pub item: Arc<T>,
	pub deco: [Option<Arc<Decoration>>; MAX_SLOTS],
	pub value: u8,
}

impl<T> Clone for AttachedDecorations<T> where T: HasDecorations + HasSkills {
	fn clone(&self) -> Self {
		AttachedDecorations {
			item: Arc::clone(&self.item),
			deco: self.deco.clone(),
			value: self.value,
		}
	}
}

impl fmt::Display for AttachedDecorations<Armor> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut decos_str = String::new();
		// ToDo use: runtime-fmt
		let mut deco_str = [String::new(), String::new(), String::new()];
		for (i, d) in self.deco.iter().enumerate() {
			deco_str[i] = {
				if let Some(deco) = d {
					format!("{} {}", self.item.slots[i], deco.to_string())
				} else {
					format!("{} None", self.item.slots[i])
				}
			}
		}
		decos_str = format!("{0: <25}|{1: <25}|{2: <25}", deco_str[0], deco_str[1], deco_str[2]);
		write!(f, "{0: <90}|{1: <77}|{2: <5}", self.item, decos_str, self.value)
	}
}

impl fmt::Debug for AttachedDecorations<Armor> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self)
	}
}

impl<T> AttachedDecorations<T> where T: HasDecorations + HasSkills {
	pub fn new(container: Arc<T>) -> Self {
		AttachedDecorations {
			item: container,
			deco: [None, None, None],
			value: 0,
		}
	}

	pub fn value(&mut self, req: &HashMap<ID, u8>) -> u8 {
		let mut value = 0;
		for skill in self.item.get_skills() {
			if req.contains_key(&skill.get_id()) {
				value += skill.level;
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

	pub fn get_item(&self) -> &Arc<T> {
		&self.item
	}

	fn is_empty(&self, i: usize) -> bool {
		self.deco[i].is_none()
	}

	pub fn try_add_deco(&mut self, deco: &Arc<Decoration>) -> Result<(), &str> {
		for (i, size) in self.item.get_slots().iter().enumerate().rev() {
			if *size >= deco.size {
				if self.is_empty(i) {
					self.deco[i] = Some(Arc::clone(deco));
					return Ok(());
				}
			}
		}
		Err("No space left")
	}

	pub fn add_skills(&self, skills_sum: &mut HashMap<ID, Level>) {
		for skill in self.item.get_skills() {
			match skills_sum.entry(skill.get_id()) {
				Entry::Occupied(mut o) => o.insert(o.get() + skill.level),
				Entry::Vacant(v) => *v.insert(skill.level)
			};
		}
		for deco in self.deco.iter() {
			if let Some(deco) = deco {
				for skill in deco.skills.get_skills() {
					match skills_sum.entry(skill.get_id()) {
						Entry::Occupied(mut o) => o.insert(o.get() + skill.level),
						Entry::Vacant(v) => *v.insert(skill.level)
					};
				}
			}
		}
	}

	pub fn get_skills(&self) -> HashMap<ID, Level> {
		let mut skills: HashMap<ID, Level> = Default::default();
		for skill in self.item.get_skills() {
			match skills.entry(skill.get_id()) {
				Entry::Occupied(mut o) => o.insert(o.get() + skill.level),
				Entry::Vacant(v) => *v.insert(skill.level)
			};
		}
		for deco in self.deco.iter() {
			if let Some(deco) = deco {
				for skill in deco.skills.get_skills() {
					match skills.entry(skill.get_id()) {
						Entry::Occupied(mut o) => o.insert(o.get() + skill.level),
						Entry::Vacant(v) => *v.insert(skill.level)
					};
				}
			}
		}
		skills
	}
}