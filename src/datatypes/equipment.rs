use std::{
    collections::HashMap,
    fmt,
    sync::Arc,
};
use strum::IntoEnumIterator;

use crate::datatypes::{
	*,
	armor::Armor,
	charm::Charm,
	decoration::AttachedDecorations,
	tool::Tool,
	weapon::Weapon,
	types::{ArmorClass, Element, Item, Wearable},
};

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

		for i in ArmorClass::iter() {
			str = format!("{0:}\t{1: <6}:", str, i.to_string());
			str = match self.set.get(i as usize).expect("ERROR: Result print out of bounds") {
				Some(armor) => format!("{} {}\n", str, armor.item.to_string()),
				None => format!("{} None\n", str),
			}
		}

		str = match &self.charm {
			Some(charm) => format!("{0:}\t{1: <6}: {2:}\n", str, "Charm", charm),
			None => format!("{0:}\t{1: <6}: None\n", str, "charm"),
		};

		for (i, tool) in self.tools.iter().enumerate() {
			str = format!("{0:}\t{1: <6}:", str, format!("tool {}", i + 1));
			str = match tool {
				Some(tool) => format!("{} {}\n", str, tool.item),
				None => format!("{} None\n", str),
			}
		}
		str.remove(str.len() - 1);
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

	fn add_or_insert(hash: &mut HashMap<ID, u8>, decorations: &Vec<Option<Arc<Decoration>>>) {
		for i in decorations {
			if let Some(j) = i {
				hash.entry(j.id)
					.and_modify(|e| { *e += 1 })
					.or_insert(1);
			}
		}
	}

	pub fn get_decorations(&self) -> HashMap<ID, u8> {
		let mut ret: HashMap<ID, u8> = Default::default();
		if let Some(w) = &self.weapon {
			Equipment::add_or_insert(&mut ret, &w.decorations);
		}
		for piece in &self.set {
			if let Some(p) = piece {
				Equipment::add_or_insert(&mut ret, &p.decorations);
			}
		}
		for tool in &self.tools {
			if let Some(t) = tool {
				Equipment::add_or_insert(&mut ret, &t.decorations);
			}
		}
		ret
	}

	pub fn get_defence(&self) -> u16 {
		let mut total: u16 = 0;
		for piece in &self.set {
			if let Some(p) = piece {
				total += p.item.defence[2] as u16;
			}
		}
		total
	}
	pub fn get_fire_defence(&self) -> i16 {
		let mut total: i16 = 0;
		for piece in &self.set {
			if let Some(p) = piece {
				total += *p.item.elements.get(Element::Fire as usize).unwrap() as i16;
			}
		}
		total
	}
	pub fn get_water_defence(&self) -> i16 {
		let mut total: i16 = 0;
		for piece in &self.set {
			if let Some(p) = piece {
				total += *p.item.elements.get(Element::Water as usize).unwrap() as i16;
			}
		}
		total
	}
	pub fn get_thunder_defence(&self) -> i16 {
		let mut total: i16 = 0;
		for piece in &self.set {
			if let Some(p) = piece {
				total += *p.item.elements.get(Element::Thunder as usize).unwrap() as i16;
			}
		}
		total
	}
	pub fn get_ice_defence(&self) -> i16 {
		let mut total: i16 = 0;
		for piece in &self.set {
			if let Some(p) = piece {
				total += *p.item.elements.get(Element::Ice as usize).unwrap() as i16;
			}
		}
		total
	}
	pub fn get_dragon_defence(&self) -> i16 {
		let mut total: i16 = 0;
		for piece in &self.set {
			if let Some(p) = piece {
				total += *p.item.elements.get(Element::Dragon as usize).unwrap() as i16;
			}
		}
		total
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

impl PartialEq for Equipment {
	fn eq(&self, other: &Self) -> bool {
		self.weapon == other.weapon &&
			self.set[0] == self.set[0] &&
			self.set[1] == self.set[1] &&
			self.set[2] == self.set[2] &&
			self.set[3] == self.set[3] &&
			self.set[4] == self.set[4] &&
			self.charm == self.charm &&
			self.tools[0] == self.tools[0] &&
			self.tools[1] == self.tools[1]
	}
}