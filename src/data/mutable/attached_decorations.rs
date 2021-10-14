use std::{
	fmt,
	sync::Arc,
};
use itertools::{
	EitherOrBoth::{Both, Left, Right},
	Itertools
};
use crate::data::db_types::{Item, Slots, decoration::Decoration, skill::SkillsLevel,};

pub struct AttachedDecorations<T> {
	pub item: Arc<T>,
	pub decorations: Vec<Option<Arc<Decoration>>>,
}

impl<T> AttachedDecorations<T> where T: Item {
	pub fn new(item: Arc<T>) -> Self {
		let item_slots = item.get_slots();
		let mut deco = vec![None; item_slots.len()];
		deco.shrink_to_fit();
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

	pub fn try_add_deco(&mut self, decoration: &Arc<Decoration>) -> bool {
		for (i, size) in self.item.get_slots().iter().enumerate().rev() {
			if *size >= decoration.size && self.is_empty(i) {
				self.set_deco(i, Arc::clone(decoration));
				return true;
			}
		}
		false
	}

	pub fn get_deco(&self, index: usize) -> Option<Arc<Decoration>> {
		if let Some(hit) = self.decorations.get(index) {
			return hit.clone();
		}
		None
	}

	fn set_deco(&mut self, index: usize, decoration: Arc<Decoration>) {
		if let Some(deco) = self.decorations.get(index) {
			let empty = deco.is_none();
			let item_slots = self.item.get_slots();
			let size = *item_slots.get(index).unwrap() >= decoration.size;
			if empty && size {  // decos.len() > index &&
				self.decorations[index] = Some(decoration);
			}
		}
	}

	pub fn clean_decorations(&mut self) {
		self.decorations = vec![None; self.item.get_slots().len()];
	}
}

impl<T> Item for AttachedDecorations<T> where T: Item {
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

	fn get_slots(&self) -> Slots {
		self.item.get_slots()
	}
}

impl<T> Clone for AttachedDecorations<T> where T: Item {
	fn clone(&self) -> Self {
		AttachedDecorations {
			item: Arc::clone(&self.item),
			decorations: self.decorations.clone(),
		}
	}
}

impl<T> PartialEq for AttachedDecorations<T> where T: PartialEq + Item {
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

impl<T> fmt::Display for AttachedDecorations<T> where T: fmt::Display + Item {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut str = String::new();
		for (i, slot) in self.item.get_slots().iter().enumerate() {
			if let Some(deco) = self.decorations.get(i).unwrap() {
				str.push_str(format!("{}", deco.get_skills()).as_str());
			} else {
				str.push_str(format!("<None [{}]>", slot).as_str());
			}
		}

		write!(f, "{0:}|{1:}", self.item, str)
	}
}