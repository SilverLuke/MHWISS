use std::{
	fmt,
};
use std::hash::{Hash, Hasher};

use crate::data::db_types::{ID, Item, MAX_SLOTS};
use crate::data::db_types::skill::SkillsLevel;

pub struct Tool {
	pub(crate) id: ID,
	pub name: String,
	slots: [u8; MAX_SLOTS],
}

impl Tool {
	pub(crate) fn new(id: ID, name: String, slots: [u8; MAX_SLOTS]) -> Self {
		Tool {
			id,
			name,
			slots,
		}
	}
}

impl Item for Tool {
	fn get_skills(&self) -> SkillsLevel {
		SkillsLevel::new()
	}

	fn has_skills(&self, _: &SkillsLevel) -> bool {
		false
	}

	fn get_slots(&self) -> Vec<u8> {
		Vec::from(self.slots)
	}
}

impl PartialEq for Tool {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}
impl Eq for Tool {}

impl Hash for Tool {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.id.hash(state);
	}
}
impl fmt::Display for Tool {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{0: <45}", self.name)
	}
}
