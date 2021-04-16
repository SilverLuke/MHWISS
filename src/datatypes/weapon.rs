use std::{
	fmt,
	sync::Arc,
	cell::RefCell,
	collections::HashMap,
};
use crate::datatypes::{
	ID, Level, MAX_SLOTS, SHARPNESS_LEVELS,
	types::{WeaponClass, Element, ElderSeal},
	skill::{Skill, SetSkill, HasSkills, SkillLevel, SkillsLevel},
	decoration::HasDecorations
};

pub struct Weapon {
	pub id: ID,
	previous_id: Option<ID>,
	pub class: WeaponClass,
	pub name: String,
	attack_true: u16,
	affinity: i8,
	sharpness: Option<[u8; SHARPNESS_LEVELS]>,
	defense: u8,
	pub slots: [u8; MAX_SLOTS],
	elements: Vec<(Element, u16)>,
	element_hidden: bool,
	elderseal: ElderSeal,
	armorset_skill: Option<Arc<SetSkill>>,
	pub skill: Option<SkillLevel>
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

	fn get_skills(&self) -> Box<dyn Iterator<Item=&SkillLevel> + '_> {
		Box::new(self.skill.iter())
	}
}

impl HasSkills for Weapon {
	fn has_skills(&self, query: &HashMap<ID, Level>) -> bool {
		todo!()
	}
	fn get_skills_rank(&self, query: &HashMap<ID, Level>) -> u8 {
		todo!()
	}
}

impl Weapon {
	pub fn new(id: ID, previous_id: Option<ID>, class: WeaponClass, name: String, attack_true: u16, affinity: i8, sharpness: Option<[u8; 7]>, defense: u8, slots: [u8; 3], elements: Vec<(Element, u16)>, element_hidden: bool, elderseal: ElderSeal, armorset_bonus_id: Option<Arc<SetSkill>>, skill: Option<SkillLevel>) -> Self {
		Weapon { id, previous_id, class, name, attack_true, affinity, sharpness, defense, slots, elements, element_hidden, elderseal, armorset_skill: armorset_bonus_id, skill }
	}
}