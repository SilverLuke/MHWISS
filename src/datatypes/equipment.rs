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
use crate::datatypes::types::{ArmorClass, Item, Wearable};
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

impl Wearable for Weapon {}
impl Wearable for Armor {}
impl Wearable for Charm {}
impl Wearable for Tool {}

impl Equipment {
	pub fn new() -> Self {
		Equipment {
			weapon: None,
			set: <[_; 5]>::default(),
			charm: None,
			tools: <[_; 2]>::default(),
		}
	}

	pub fn try_add_weapon(&mut self, weapon: AttachedDecorations<Weapon>) -> Result<(), &str> {
		if self.weapon.is_some() {
			Err("Space already taken")
		} else {
			println!("Added:\t{}", &weapon.item);
			self.weapon = Some(weapon);
			Ok(())
		}
	}

	pub fn try_add_armor(&mut self, armor: AttachedDecorations<Armor>) -> Result<(), &str> {
		let i = armor.item.class as usize;
		if self.set[i].is_some() {
			Err("Space already taken")
		} else {
			println!("Added:\t{}", &armor.item);
			self.set[i] = Some(armor);
			Ok(())
		}
	}

	pub fn try_add_charm(&mut self, charm: Arc<Charm>) -> Result<(), &str> {
		if self.charm.is_some() {
			Err("Space already taken")
		} else {
			println!("Added:\t{}", &charm);
			self.charm = Some(charm);
			Ok(())
		}
	}

	pub fn try_add_tool(&mut self, tool: AttachedDecorations<Tool>) -> Result<(), &str> {
		let mut index = None;
		for (i, t) in &mut self.tools.iter().enumerate() {
			if t.is_none() {
				index = Some(i);
				println!("Added:\t{}", tool.item);
			}
		}
		if let Some(i) = index {
			self.tools[i] = Some(tool);
			return Ok(());
		} else {
			Err("Space already taken")
		}
	}

	fn add_weapon_skills(&self, skills_sum: &mut HashMap<ID, Level>) {
		if let Some(weapon) = &self.weapon {
			weapon.get_skills_chained(skills_sum);
		}
	}

	fn add_armors_skills(&self, skills_sum: &mut HashMap<ID, Level>) {
		for i in self.set.iter() {
			if let Some(armor) = i {
				armor.get_skills_chained(skills_sum);
			}
		}
	}

	fn add_charm_skills(&self, skills_sum: &mut HashMap<ID, Level>) {
		if let Some(charm) = &self.charm {
			charm.get_skills_chained(skills_sum);
		}
	}

	pub fn get_skills(&self) -> HashMap<ID, Level> {  // TODO implemets Item trait ??
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