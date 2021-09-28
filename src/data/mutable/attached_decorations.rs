use std::{
	fmt,
	sync::Arc,
};
use itertools::{
	EitherOrBoth::{Both, Left, Right},
	Itertools
};
use crate::data::db_types::{HasDecoration, armor::Armor, tool::Tool, decoration::Decoration, skill::SkillsLevel, HasSkills};

pub struct AttachedDecorations<T> {
	pub item: Arc<T>,
	pub decorations: Vec<Option<Arc<Decoration>>>,
}

impl<T> Clone for AttachedDecorations<T> where T: HasDecoration {
	fn clone(&self) -> Self {
		AttachedDecorations {
			item: Arc::clone(&self.item),
			decorations: self.decorations.clone(),
		}
	}
}

impl<T> AttachedDecorations<T> where T: HasDecoration {
	pub fn new(item: Arc<T>) -> Self {
		let item_slots = HasDecoration::get_slots(item.as_ref());
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
		for (i, size) in HasDecoration::get_slots(self.item.as_ref()).iter().enumerate().rev() {
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
			let item_slots = HasDecoration::get_slots(self.item.as_ref());
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
}

impl<T> HasSkills for AttachedDecorations<T> where T: HasSkills + HasDecoration {
	fn get_skills(&self) -> SkillsLevel {
		let mut ret = self.item.get_skills();
		for decoration in self.decorations.iter().flatten() {
			ret.insert_skills(&decoration.get_skills());
		}
		ret
	}

	fn has_skills(&self, query: &SkillsLevel) -> bool {
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
}

impl HasSkills for AttachedDecorations<Tool> {
	fn get_skills(&self) -> SkillsLevel {
		let mut ret = SkillsLevel::new();
		for decoration in self.decorations.iter().flatten() {
			ret.insert_skills(&decoration.get_skills());
		}
		ret
	}

	fn has_skills(&self, query: &SkillsLevel) -> bool {
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

impl fmt::Display for AttachedDecorations<Armor> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// ToDo use: runtime-fmt
		let str =
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

impl<T> PartialEq for AttachedDecorations<T> where T: PartialEq + HasDecoration {
	fn eq(&self, other: &Self) -> bool {
		if self.decorations.len() == other.decorations.len() {
			let mut equals = true;
			for it in self.decorations.iter().zip_longest(other.decorations.iter()) {
				equals &= match it {
					Both(x, y) => x == y,
					Left(_x) => unreachable!(),
					Right(_y) => unreachable!(),
				}
			}
			self.item == other.item && equals
		} else {
			false
		}
	}
}