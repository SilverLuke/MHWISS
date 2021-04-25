use std::{
	fmt,
	sync::Arc,
	cell::RefCell,
	collections::HashMap,
};
use crate::datatypes::{
	ID, Level, MAX_SLOTS, SHARPNESS_LEVELS,
	types::{WeaponClass, Element, ElderSeal},
	skill::{Skill, SetSkill, SkillLevel, SkillsLevel},
	types::Item,
};
use std::collections::hash_map::Entry;

pub struct Weapon {
	pub id: ID,
	previous_id: Option<ID>,
	pub class: WeaponClass,
	pub name: String,
	pub attack_true: u16,
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

impl Weapon {
	pub fn new(id: ID, previous_id: Option<ID>, class: WeaponClass, name: String, attack_true: u16, affinity: i8, sharpness: Option<[u8; 7]>, defense: u8, slots: [u8; 3], elements: Vec<(Element, u16)>, element_hidden: bool, elderseal: ElderSeal, armorset_bonus_id: Option<Arc<SetSkill>>, skill: Option<SkillLevel>) -> Self {
		Weapon { id, previous_id, class, name, attack_true, affinity, sharpness, defense, slots, elements, element_hidden, elderseal, armorset_skill: armorset_bonus_id, skill }
	}
}

impl Item for Weapon {
	fn has_skills(&self, query: &HashMap<ID, Level>) -> bool {
		if let Some(skill) = &self.skill {
			return query.contains_key(&skill.get_id());
		}
		false
	}

	fn get_skills_chained(&self, chained: &mut HashMap<ID, Level>) {
		if let Some(skill) = &self.skill {
			match chained.entry(skill.get_id()) {
				Entry::Occupied(mut o) => o.insert(o.get() + skill.level),
				Entry::Vacant(v) => *v.insert(skill.level)
			};
		}
	}

	fn get_skills_hash(&self) -> HashMap<ID, Level> {
		let mut ret: HashMap<ID, Level> = Default::default();
		if let Some(skill) = &self.skill {
			ret.insert(skill.get_id(), skill.level);
		}
		ret
	}

	fn get_skills_iter(&self) -> Box<dyn Iterator<Item=&SkillLevel> + '_> {
		Box::new(self.skill.iter())
	}

	fn get_slots(&self) -> Option<Vec<u8>> {
		Some(Vec::from(self.slots))
	}
}

impl fmt::Display for Weapon {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}[{}]", self.name, self.id)
	}
}