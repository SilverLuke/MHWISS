use crate::forge::types::{ID, Element, WeaponClass, SkillLev};
use crate::forge::skill::{SetSkill};
use std::rc::Rc;
use std::{fmt};
use crate::searcher::container::{HasDecorations, HasSkills};
use std::collections::hash_map::RandomState;
use std::cell::RefCell;
use std::collections::HashMap;

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
	pub skill: Option<SkillLev>
}

impl fmt::Display for Weapon {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

		write!(f, "{}[{}]", self.name, self.id)
	}
}

impl HasDecorations for Weapon {
	fn get_slots(&self) -> [u8; 3] {
		self.slots
	}

	fn get_skills(&self) -> Box<dyn Iterator<Item=&SkillLev> + '_> {
		Box::new(self.skill.iter())
	}
}

impl HasSkills for Weapon {
	fn has_skills(&self, query: &RefCell<HashMap<u16, u8, RandomState>>) -> bool {
		todo!()
	}
}

impl Weapon {
	pub fn new(id: ID, previous_id: Option<ID>, class: WeaponClass, name: String, attack_true: u16, affinity: i8, sharpness: Option<[u8; 7]>, defense: u8, slots: [u8; 3], elements: Vec<(Element, u16)>, element_hidden: bool, elderseal: u8, armorset_bonus_id: Option<Rc<SetSkill>>, skill: Option<SkillLev>) -> Self {
		Weapon { id, previous_id, class, name, attack_true, affinity, sharpness, defense, slots, elements, element_hidden, elderseal, armorset_skill: armorset_bonus_id, skill }
	}
}