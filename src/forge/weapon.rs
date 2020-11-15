use crate::forge::types::{
	ID,	Element
};
struct Weapon {
	id: ID,
	name: String,
	attack: u16,
	affiniy: i8,
	element: Vec<(Element,u8)>,
	slots: u8,  //
}

impl Weapon {
	pub fn new() -> Self {
		Weapon{
			id: 1,
			name: "Implement".to_string(),
			attack: 0,
			affiniy: 0,
			element: Vec::new(),
			slots: 0
		}
	}
}