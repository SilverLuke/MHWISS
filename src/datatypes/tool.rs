use crate::datatypes::ID;
use crate::datatypes::decoration::HasDecorations;
use std::rc::Rc;
use crate::datatypes::skill::{Skill, HasSkills};
use std::cell::RefCell;
use std::collections::HashMap;

pub struct Tool {
	id: ID,
	pub name: String,
	slots: [u8; 2],
}

impl Tool {
	fn new(id:ID, name:String) -> Self {
		Tool {
			id: 1,
			name: String::from("Test"),
			slots: [1,4],
		}
	}
}

impl HasDecorations for Tool {
	fn get_slots(&self) -> [u8; 3] {
		todo!()
	}

	fn get_skills(&self) -> Box<dyn Iterator<Item=&(Rc<Skill>, u8)>> {
		todo!()
	}
}

impl HasSkills for Tool {
	fn has_skills(&self, query: &RefCell<HashMap<u16, u8>>) -> bool {
		todo!()
	}

	fn get_skills_rank(&self, query: &RefCell<HashMap<u16, u8>>) -> Option<u8> {
		todo!()
	}
}