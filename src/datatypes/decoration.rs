use std::{
	fmt,
	sync::Arc,
	cell::RefCell,
	collections::{HashMap, hash_map::Entry},
};
use itertools::{
	Itertools,
	EitherOrBoth::{Both, Left, Right}
};
use crate::datatypes::{
	ID, Level, MAX_SLOTS,
	types::{Item, Decorable},
	skill::{Skill, SkillLevel, SkillsLevel},
	armor::Armor,
	weapon::Weapon,
	tool::Tool,
};
use std::borrow::Borrow;

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

impl PartialEq for Decoration {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

pub struct AttachedDecorations<T: Item + Decorable> {
	pub item: Arc<T>,
	pub decorations: Vec<Option<Arc<Decoration>>>,
}

impl<T> Clone for AttachedDecorations<T> where T: Item + Decorable {
	fn clone(&self) -> Self {
		AttachedDecorations {
			item: Arc::clone(&self.item),
			decorations: self.decorations.clone(),
		}
	}
}

impl<T> AttachedDecorations<T> where T: Item + Decorable {
	pub fn new(item: Arc<T>) -> Self {
		let item_slots = Decorable::get_slots(item.as_ref());
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
		for (i, size) in  Decorable::get_slots(self.item.as_ref()).iter().enumerate().rev() {
			if *size >= decoration.size && self.is_empty(i) {
				self.set_deco(i, decoration);
				return Ok(());
			}
		}
		Err("No space left")
	}

	pub(crate) fn get_deco(&self, index: usize) -> Option<Arc<Decoration>> {
		if let Some(hit) = self.decorations.get(index) {
			return hit.clone();
		}
		None
	}

	fn set_deco(&mut self, index: usize, decoration: Arc<Decoration>) {
		if let Some(deco) = self.decorations.get(index) {
			let empty = deco.is_none();
			let item_slots = Decorable::get_slots(self.item.as_ref());
			let size = *item_slots.get(index).unwrap() >= decoration.size;
			if empty && size {  // decos.len() > index &&
				self.decorations[index] = Some(decoration);
			}
		}
	}

	fn replace_deco(&mut self, index: usize, decoration: Arc<Decoration>) -> Option<Arc<Decoration>> {
		if index < self.decorations.len() {
			std::mem::replace(&mut self.decorations[index], Some(decoration))
		} else {
			None
		}
	}

	fn get_slots(&self) -> Option<Vec<u8>> {
		Item::get_slots(self)
	}
}

impl<T> Item for AttachedDecorations<T> where T: Item + Decorable {
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
		Item::get_slots(self.item.borrow() as &T)
	}
}

impl<T> PartialEq for AttachedDecorations<T> where T: PartialEq + Item + Decorable {
	fn eq(&self, other: &Self) -> bool {
		if self.decorations.len() == other.decorations.len() {
			let mut equals = true;
			for it in self.decorations.iter().zip_longest(other.decorations.iter()) {
				equals &= match it {
					Both(x, y) => x == y,
					Left(x) => unreachable!(),
					Right(y) => unreachable!(),
				}
			}
			self.item == other.item && equals
		} else {
			false
		}
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
