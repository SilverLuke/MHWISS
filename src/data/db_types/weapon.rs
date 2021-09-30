use std::{
    fmt,
    sync::Arc,
};
use std::hash::{Hash, Hasher};

use crate::data::db_types::{
	ID, MAX_SLOTS, SHARPNESS_LEVELS, HasDecoration, ElderSeal, Element, WeaponClass, HasSkills,
	skill::{SetSkill, SkillsLevel}
};

#[allow(dead_code)]
pub struct Weapon {
	pub id: ID,
	previous_id: Option<ID>,
	pub class: WeaponClass,
	pub name: String,
	pub attack_true: u16,
	affinity: i8,
	sharpness: Option<[u8; SHARPNESS_LEVELS]>,
	defense: u8,
	elements: Vec<(Element, u16)>,
	element_hidden: bool,
	elderseal: ElderSeal,
	pub skill: SkillsLevel,
	pub slots: [u8; MAX_SLOTS],
	armorset_skill: Option<Arc<SetSkill>>,
}

impl Weapon {
	pub fn new(id: ID, previous_id: Option<ID>, class: WeaponClass, name: String, attack_true: u16, affinity: i8, sharpness: Option<[u8; 7]>, defense: u8, slots: [u8; 3], elements: Vec<(Element, u16)>, element_hidden: bool, elderseal: ElderSeal, armorset_bonus_id: Option<Arc<SetSkill>>, skill: SkillsLevel) -> Self {
		Weapon { id, previous_id, class, name, attack_true, affinity, sharpness, defense, slots, elements, element_hidden, elderseal, armorset_skill: armorset_bonus_id, skill }
	}
}

impl HasSkills for Weapon {
	fn get_skills(&self) -> SkillsLevel {
		self.skill.clone()
	}
}

impl HasDecoration for Weapon {
	fn get_slots(&self) -> Vec<u8> {
		Vec::from(self.slots)
	}
}

impl PartialEq for Weapon {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}
impl Eq for Weapon {}

impl Hash for Weapon {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.id.hash(state);
	}
}

impl fmt::Display for Weapon {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}[{}]", self.name, self.id)
	}
}
