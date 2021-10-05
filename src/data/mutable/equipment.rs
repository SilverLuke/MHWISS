use std::{
	collections::HashMap,
	fmt,
	sync::Arc,
};
use strum::IntoEnumIterator;
use crate::data::{
	db_types::{
		*,
		skill::SkillsLevel,
		weapon::Weapon,
		armor::Armor,
		charm::Charm,
		tool::Tool,
		decoration::Decoration,
	},
	mutable::attached_decorations::AttachedDecorations,
};

pub struct Equipment {
	pub weapon: Option<AttachedDecorations<Weapon>>,
	pub set: [Option<AttachedDecorations<Armor>>; 5],
	pub charm: Option<Arc<Charm>>,
	pub tools: [Option<AttachedDecorations<Tool>>; 2],
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

	pub fn evaluate(&self, requirements: SkillsLevel) -> u16 {
		let mut value = 0;
		for skill in self.get_skills().iter() {
			if let Some(level) = requirements.get_level(skill.get_skill()) {
				value += level as u16;
			}
		}
		value
	}

	pub fn try_add_weapon(&mut self, weapon: AttachedDecorations<Weapon>) -> bool {
		if self.weapon.is_some() {
			false
		} else {
			self.weapon = Some(weapon);
			true
		}
	}

	pub fn try_add_armor(&mut self, armor: AttachedDecorations<Armor>) -> bool {
		let i = armor.item.class as usize;
		if self.set[i].is_some() {
			false
		} else {
			self.set[i] = Some(armor);
			true
		}
	}

	pub fn try_add_charm(&mut self, charm: Arc<Charm>) -> bool {
		if self.charm.is_some() {
			false
		} else {
			self.charm = Some(charm);
			true
		}
	}

	pub fn try_add_tool(&mut self, tool: AttachedDecorations<Tool>) -> bool {
		for t in self.tools.iter_mut() {
			if t.is_none() {
				*t = Some(tool);
				return true;
			}
		}
		false
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

	pub fn get_used_decorations(&self) -> HashMap<Arc<Decoration>, u8> {
		fn add_or_insert(collection: &mut HashMap<Arc<Decoration>, u8>, decorations: &Vec<Option<Arc<Decoration>>>) {
			for i in decorations {
				if let Some(j) = i {
					collection.entry(Arc::clone(j))
						.and_modify(|e| { *e += 1 })
						.or_insert(1);
				}
			}
		}
		let mut ret: HashMap<Arc<Decoration>, u8> = Default::default();
		if let Some(w) = &self.weapon {
			add_or_insert(&mut ret, &w.decorations);
		}
		for piece in &self.set {
			if let Some(p) = piece {
				add_or_insert(&mut ret, &p.decorations);
			}
		}
		for tool in &self.tools {
			if let Some(t) = tool {
				add_or_insert(&mut ret, &t.decorations);
			}
		}
		ret
	}
}

impl Item for Equipment {
	fn get_skills(&self) -> SkillsLevel {
		let mut ret = SkillsLevel::new();
		if let Some(weapon) = &self.weapon {
			ret.insert_skills(&weapon.get_skills());
		}
		for i in self.set.iter() {
			if let Some(armor) = i {
				ret.insert_skills(&armor.get_skills());
			}
		}
		if let Some(charm) = &self.charm {
			ret.insert_skills(&charm.get_skills());
		}
		for i in self.tools.iter() {
			if let Some(tool) = i {
				ret.insert_skills(&tool.get_skills());
			}
		}
		ret.shrink_to_fit();
		ret
	}

	fn get_slots(&self) -> Slots {
		let mut ret : Slots = vec![];
		if let Some(weapon) = &self.weapon {
			ret.append(weapon.get_slots().as_mut());
		}
		for i in self.set.iter() {
			if let Some(armor) = i {
				ret.append(armor.get_slots().as_mut());
			}
		}
		for i in self.tools.iter() {
			if let Some(tool) = i {
				ret.append(tool.get_slots().as_mut());
			}
		}
		ret.shrink_to_fit();
		ret
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

impl fmt::Display for Equipment {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut str;
		str = match &self.weapon {
			Some(w) => format!("\t{0: <6}: {1:}\n", "weapon", w),
			None => format!("\tweapon: None\n")
		};

		for i in ArmorClass::iter() {
			str = format!("{0:}\t{1: <6}:", str, i.to_string());
			str = match self.set.get(i as usize).expect("ERROR: Result print out of bounds") {
				Some(armor) => format!("{} {}\n", str, armor),
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
				Some(tool) => format!("{} {}\n", str, tool),
				None => format!("{} None\n", str),
			}
		}
		str.remove(str.len() - 1);
		write!(f, "{}", str)
	}
}