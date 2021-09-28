use std::{
	fmt,
};

use crate::data::db_types::{ID, HasDecoration, MAX_SLOTS};

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

impl HasDecoration for Tool {
	fn get_slots(&self) -> Vec<u8> {
		Vec::from(self.slots)
	}
}

impl fmt::Display for Tool {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.name)
	}
}

impl PartialEq for Tool {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}