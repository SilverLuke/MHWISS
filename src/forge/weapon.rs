use crate::forge::types::{ID, Element, WeaponClass};
use crate::forge::skill::{SetSkill, Skill};
use std::rc::Rc;
use std::fmt;

pub struct Weapon {
	pub id: ID,
	previous_id: Option<ID>,
	class: WeaponClass,
	name: String,
	attack_true: u16,
	affinity: i8,
	sharpness: Option<[u8;7]>,
	defense: u8,
	slots: [u8; 3],
	elements: Vec<(Element, u16)>,
	element_hidden: bool,
	elderseal: u8,
	armorset_skill: Option<Rc<SetSkill>>,
	pub skill: Option<Rc<Skill>>
}

impl fmt::Display for Weapon {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

		write!(f, "{}[{}]", self.name, self.id)
	}
}

impl Weapon {
	pub fn new(id: ID, previous_id: Option<ID>, class: WeaponClass, name: String, attack_true: u16, affinity: i8, sharpness: Option<[u8; 7]>, defense: u8, slots: [u8; 3], elements: Vec<(Element, u16)>, element_hidden: bool, elderseal: u8, armorset_bonus_id: Option<Rc<SetSkill>>, skilltree_id: Option<Rc<Skill>>) -> Self {
		Weapon { id, previous_id, class, name, attack_true, affinity, sharpness, defense, slots, elements, element_hidden, elderseal, armorset_skill: armorset_bonus_id, skill: skilltree_id }
	}
}