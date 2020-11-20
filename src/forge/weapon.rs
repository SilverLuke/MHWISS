use crate::forge::types::{ID, Element, WeaponClass};

pub struct Weapon {
	id: ID,
	name: String,
	class: WeaponClass,
	attack: u16,
	affiniy: i8,
	element: Vec<(u8)>,
	slots: u8,  //
}

impl Weapon {
	pub fn new() -> Self {
		Weapon{
			id: 1,
			name: "Implement".to_string(),
			class: WeaponClass::Bow,
			attack: 0,
			affiniy: 0,
			element: Vec::new(),
			slots: 0
		}
	}
}