use std::{
	fmt,
	rc::Rc,
	ops::Not,
	sync::Arc,
	cell::RefCell,
	collections::{
		hash_map::Entry,
		HashMap,
	},
};
use itertools::Itertools;
use crate::datatypes::*;
use std::borrow::Borrow;
use crate::datatypes::weapon::Weapon;
use crate::datatypes::armor::Armor;
use crate::datatypes::charm::Charm;
use crate::datatypes::types::ArmorClass;
use crate::datatypes::decoration::AttachedDecorations;
use crate::datatypes::tool::Tool;

pub struct BestSet {
	pub weapon: Option<AttachedDecorations<Weapon>>,
	pub set: [Option<AttachedDecorations<Armor>>; 5],
	pub charm: Option<Rc<Charm>>,
	pub tools: [Option<AttachedDecorations<Tool>>; 2],
}

impl fmt::Display for BestSet {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut str;
		str = match &self.weapon {
			Some(w) => format!("Weapon: {}\n", w.item),
			None => format!("Weapon: None\n")
		};

		for i in ArmorClass::iterator() {
			str = format!("{} {}:", str, ArmorClass::to_string(i));
			match self.set.get(*i as usize) {
				Some(Some(armor)) => str = format!("{} <{}>\n", str, armor.item),
				Some(None) => str = format!("{} None\n", str),
				None => panic!("ERROR: Result print out of bounds"),
			}
		}

		if let Some(charm) = &self.charm {
			str = format!("{} Charm: {}", str, charm);
		}

		write!(f, "{}", str)
	}
}

impl BestSet {
	pub fn new() -> Self {
		BestSet {
			weapon: None,
			set: <[_; 5]>::default(),
			charm: None,
			tools: <[_; 2]>::default(),
		}
	}

	pub fn try_add_armor(&mut self, armor: &AttachedDecorations<Armor>) -> Result<(), &str> {
		let i = armor.item.class as usize;
		if self.set[i].is_some() {
			Err("Space already taken")
		} else {
			self.set[i] = Some(armor.clone());
			println!("Added: {}", armor.item);
			Ok(())
		}
	}

	fn add_weapon_skills(&self, skills_sum: &mut HashMap<ID, Level>) {
		if let Some(deco_container) = &self.weapon {
			if let Some(skill) = &deco_container.item.skill {
				*skills_sum.entry(skill.0.id).or_insert(1) += 1;
			}
		}
	}

	fn add_armors_skills(&self, skills_sum: &mut HashMap<ID, Level>) {
		for i in self.set.iter() {
			if let Some(armor) = i {
				armor.add_skills(skills_sum);
			}
		}
	}

	fn add_charm_skills(&self, skills_sum: &mut HashMap<ID, Level>) {
		if let Some(charm) = &self.charm {
			for (skill, lev) in charm.skills.iter() {
				*skills_sum.entry(skill.id).or_insert(*lev) += *lev;
			}
		}
	}

	pub fn get_skills(&self) -> HashMap<ID, Level> {
		let mut skills_sum: HashMap<ID, Level> = Default::default();
		self.add_weapon_skills(&mut skills_sum);
		self.add_armors_skills(&mut skills_sum);
		self.add_charm_skills(&mut skills_sum);
		skills_sum.shrink_to_fit();
		skills_sum
	}

	pub fn is_full(&self) -> bool {
		let mut count = 0;
		for i in self.set.iter() {
			if i.is_some() {
				count += 1;
			}
		}
		let tmp = self.set.len();
		count == tmp
	}
}