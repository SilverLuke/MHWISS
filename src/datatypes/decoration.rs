use std::{
	fmt,
	sync::Arc,
	cell::RefCell,
	collections::{HashMap, hash_map::Entry},
};
use crate::datatypes::{
	ID, Level, MAX_SLOTS,
	types::Item,
	skill::{Skill, SkillLevel, SkillsLevel},
	armor::Armor
};
use crate::datatypes::types::Decorable;
use crate::datatypes::weapon::Weapon;
use crate::datatypes::tool::Tool;

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

impl Item for Decoration {
	fn has_skills(&self, query: &HashMap<ID, Level>) -> bool {
		self.skills.contains_hash(query)
	}

	fn get_skills_chained(&self, chained: &mut HashMap<ID, Level>) {
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

pub struct AttachedDecorations<T: Item> {
	pub item: Arc<T>,
	pub decorations: Vec<Option<Arc<Decoration>>>,
}

impl<T> Clone for AttachedDecorations<T> where T: Item {
	fn clone(&self) -> Self {
		AttachedDecorations {
			item: Arc::clone(&self.item),
			decorations: self.decorations.clone(),
		}
	}
}

impl Decorable for Weapon {}

impl Decorable for Armor {}

impl Decorable for Tool {}

impl<T> AttachedDecorations<T> where T: Item + Decorable {
	pub fn new(item: Arc<T>) -> Self {
		let item_slots = item.get_slots().unwrap();
		let deco = vec![None; item_slots.len()];
		AttachedDecorations {
			item,
			decorations: deco,
		}
	}

	pub fn get_item(&self) -> &Arc<T> {
		&self.item
	}

	fn is_empty(&self, i: usize) -> bool {
		if let Some(hit) = self.decorations.get(i) {
			return hit.is_none();
		}
		false
	}

	pub fn try_add_deco(&mut self, decoration: Arc<Decoration>) -> Result<(), &str> {
		if let Some(slots) = self.item.get_slots() {
			for (i, size) in slots.iter().enumerate().rev() {
				if *size >= decoration.size && self.is_empty(i) {
					self.set_deco(i, decoration);
					return Ok(());
				}
			}
			Err("No space left")
		} else {
			Err("This object do not have slots")
		}
	}

	pub(crate) fn get_deco(&self, index: usize) -> Option<Arc<Decoration>> {
		if let Some(hit) = self.decorations.get(index) {
			return hit.clone();  // TODO: this .clone() is right???
		}
		None
	}

	fn set_deco(&mut self, index: usize, decoration: Arc<Decoration>) {
		let empty = self.decorations.get(index).unwrap().is_none();  // TODO to many unwrap ?
		let size = *self.item.get_slots().unwrap().get(index).unwrap() >= decoration.size;
		if empty && size {  // decos.len() > index &&
			self.decorations[index] = Some(decoration);
		}
	}

	fn replace_deco(&mut self, index: usize, decoration: Arc<Decoration>) -> Option<Arc<Decoration>> {
		if index < self.decorations.len() {
			std::mem::replace(&mut self.decorations[index], Some(decoration))
		} else {
			None
		}
	}
}

impl<T> Item for AttachedDecorations<T> where T: Item {
	fn has_skills(&self, query: &HashMap<ID, Level>) -> bool {
		self.item.has_skills(query) || {
			for deco in self.decorations.iter() {
				if let Some(deco) = deco {
					if deco.has_skills(query) {
						return true;
					}
				}
			}
			false
		}
	}

	fn get_skills_chained(&self, chained: &mut HashMap<ID, Level>) {
		self.item.get_skills_chained(chained);
		for deco in self.decorations.iter() {
			if let Some(deco) = deco {
				deco.get_skills_chained(chained);
			}
		}
	}

	fn get_skills_hash(&self) -> HashMap<ID, Level> {
		let mut ret = Default::default();
		self.item.get_skills_chained(&mut ret);
		for deco in self.decorations.iter() {
			if let Some(deco) = deco {
				deco.get_skills_chained(&mut ret);
			}
		}
		ret
	}

	fn get_skills_iter(&self) -> Box<dyn Iterator<Item=&SkillLevel>> {
		todo!()
	}

	fn get_slots(&self) -> Option<Vec<u8>> {
		self.item.get_slots()
	}
}

impl fmt::Display for AttachedDecorations<Armor> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// ToDo use: runtime-fmt
		let mut str =
			if self.decorations.len() > 0 {
				let mut deco_str = [String::new(), String::new(), String::new()];  // TODO refactor this shit please
				for (i, d) in self.decorations.iter().enumerate() {
					deco_str[i] = {
						if let Some(deco) = d {
							format!("{} {}", self.item.slots[i], deco.to_string())
						} else {
							format!("{} None", self.item.slots[i])
						}
					}
				}
				format!("{0: <25}|{1: <25}|{2: <25}", deco_str[0], deco_str[1], deco_str[2])
			} else {
				String::from("None")
			};
		write!(f, "{0: <90}|{1: <77}", self.item, str)
	}
}

impl fmt::Debug for AttachedDecorations<Armor> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self)
	}
}