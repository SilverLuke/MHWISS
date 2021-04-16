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

pub struct Equipment {
	pub weapon: Option<AttachedDecorations<Weapon>>,
	pub set: [Option<AttachedDecorations<Armor>>; 5],
	pub charm: Option<Arc<Charm>>,
	pub tools: [Option<AttachedDecorations<Tool>>; 2],
}

impl fmt::Display for Equipment {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut str;
		str = match &self.weapon {
			Some(w) => format!("\t{0: <6}: {1:}\n", "weapon", w.item),
			None => format!("\tweapon: None\n")
		};

		for i in ArmorClass::iterator() {
			str = format!("{0:}\t{1: <6}:", str, i.to_string());
			str = match self.set.get(*i as usize).expect("ERROR: Result print out of bounds") {
				Some(armor) => format!("{} {}\n", str, armor.item),
				None => format!("{} None\n", str),
			}
		}

		str = match &self.charm {
			Some(charm) => format!("{0:}\t{1: <6}: {2:}\n", str, "Charm", charm),
			None => format!("{0:}\t{1: <6}: None\n", str, "charm"),
		};

		for (i, tool) in self.tools.iter().enumerate() {
			str = format!("{0:}\t{1: <6}:", str, format!("tool {}", i+1));
			str = match tool {
				Some(tool) => format!("{} {}\n", str, tool.item),
				None => format!("{} None\n", str),
			}
		}
		str.remove(str.len()-1);
		write!(f, "{}", str)
	}
}

impl Equipment {
	pub fn new() -> Self {
		Equipment {
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
			println!("Added:\n\t{}", armor.item);
			Ok(())
		}
	}

	fn add_weapon_skills(&self, skills_sum: &mut HashMap<ID, Level>) {
		if let Some(deco_container) = &self.weapon {
			if let Some(skill) = &deco_container.item.skill {
				*skills_sum.entry(skill.get_id()).or_insert(1) += 1;
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
			for skill in charm.skills.get_skills() {
				*skills_sum.entry(skill.get_id()).or_insert(skill.level) += skill.level;
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