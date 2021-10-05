use std::{
	collections::HashSet,
	slice::Iter,
	sync::Arc,
};
use strum::{Display, EnumIter, EnumString, EnumCount};
use crate::data::db_types::{
	armor::{Armor, ArmorSet},
	charm::Charm,
	tool::Tool,
	decoration::Decoration,
	skill::{SetSkill, Skill, SkillsLevel},
	weapon::Weapon,
	Element::{Blast, Dragon, Fire, Ice, Paralysis, Poison, Sleep, Stun, Thunder, Water},
};

pub mod armor;
pub mod charm;
pub mod decoration;
pub mod skill;
pub mod tool;
pub mod weapon;

pub type ID = u16;
pub type Level = u8;
pub type Slot = u8;

pub type Weapons =     HashSet<Arc<Weapon>>;
pub type Armors =      HashSet<Arc<Armor>>;
pub type Charms =      HashSet<Arc<Charm>>;
pub type Sets =        HashSet<Arc<ArmorSet>>;
pub type Decorations = HashSet<Arc<Decoration>>;
pub type Skills =      HashSet<Arc<Skill>>;
pub type SetSkills =   HashSet<Arc<SetSkill>>;
pub type Tools =       HashSet<Arc<Tool>>;
pub type Slots =       Vec<Slot>;

pub const MAX_SLOTS: usize = 3;
pub const SHARPNESS_LEVELS: usize = 7;

pub trait Item {
	fn get_skills(&self) -> SkillsLevel;
	fn has_skills(&self, query: &SkillsLevel) -> bool {
		self.get_skills().contains_list(query)
	}
	fn get_slots(&self) -> Slots;
}

pub trait Wearable {
	fn get_name(&self) -> String;
}

// Elements
#[derive(Display, EnumString, EnumIter)]
pub enum Element {
	#[strum(serialize = "fire")]
	Fire,
	#[strum(serialize = "water")]
	Water,
	#[strum(serialize = "thunder")]
	Thunder,
	#[strum(serialize = "ice")]
	Ice,
	#[strum(serialize = "dragon")]
	Dragon,
	#[strum(serialize = "poison")]
	Poison,
	#[strum(serialize = "sleep")]
	Sleep,
	#[strum(serialize = "paralysis")]
	Paralysis,
	#[strum(serialize = "blast")]
	Blast,
	#[strum(serialize = "stun")]
	Stun,
}

impl Element {
	pub fn iter_element() -> Iter<'static, Element> {
		static ELEMENTS: [Element; 5] = [Fire, Water, Thunder, Ice, Dragon];
		ELEMENTS.iter()
	}
	#[allow(dead_code)]
	pub fn iter_status() -> Iter<'static, Element> {
		static ELEMENTS: [Element; 5] = [Poison, Sleep, Paralysis, Blast, Stun];
		ELEMENTS.iter()
	}

	pub const fn len() -> usize {
		// Equal to do this:
		// static ELEMENTS: [Element; 5] = [Fire, Water, Thunder, Ice, Dragon];
		// ELEMENTS.len()
		5
	}
}

// Armor Class
#[repr(usize)]
#[derive(EnumCount, EnumString, EnumIter, Display, Copy, Clone)]
pub enum ArmorClass {
	#[strum(serialize = "head")]
	Head,
	#[strum(serialize = "chest")]
	Chest,
	#[strum(serialize = "arms")]
	Arms,
	#[strum(serialize = "waist")]
	Waist,
	#[strum(serialize = "legs")]
	Legs,
}

// ArmorSet rank level
#[repr(usize)]
#[derive(EnumCount, EnumString, EnumIter, Display, Copy, Clone)]
pub enum ArmorRank {
	#[strum(serialize = "LR")]
	Low,
	#[strum(serialize = "HR")]
	High,
	#[strum(serialize = "MR")]
	Master,
}

// Armor related. There are some armors only for some gender
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Gender {
	Male,
	Female,
	All,
}

impl Gender {
	pub fn new(male: bool, female: bool) -> Self {
		match (male, female) {
			(false, false) => panic!("No gender"),
			(true, false) => Gender::Male,
			(false, true) => Gender::Female,
			(true, true) => Gender::All,
		}
	}
}

// Weapon type
#[repr(usize)]
#[derive(EnumCount, EnumString, EnumIter, Display, Copy, Clone)]
pub enum WeaponClass {
	#[strum(serialize = "bow")]
	Bow,
	#[strum(serialize = "charge-blade")]
	ChargeBlade,
	#[strum(serialize = "dual-blades")]
	DualBlade,
	#[strum(serialize = "great-sword")]
	GreatSword,
	#[strum(serialize = "gunlance")]
	Gunlance,
	#[strum(serialize = "hammer")]
	Hammer,
	#[strum(serialize = "heavy-bowgun")]
	HeavyBowgun,
	#[strum(serialize = "hunting-horn")]
	HuntingHorn,
	#[strum(serialize = "insect-glaive")]
	InsectGlaive,
	#[strum(serialize = "lance")]
	Lance,
	#[strum(serialize = "light-bowgun")]
	LightBowgun,
	#[strum(serialize = "long-sword")]
	Longsword,
	#[strum(serialize = "switch-axe")]
	SwitchAxe,
	#[strum(serialize = "sword-and-shield")]
	SwordAndShield,
}

// Elder Seal level only for weapons
pub enum ElderSeal {
	Empty,
	Low,
	Medium,
	High,
}

impl ElderSeal {
	pub fn new(lev: Option<String>) -> Self {
		if lev.is_none() {
			ElderSeal::Empty
		} else {
			match lev.unwrap().as_str() {
				"low" => ElderSeal::Low,
				"medium" => ElderSeal::Medium,
				"high" => ElderSeal::High,
				_ => panic!("Elderseal parsing error")
			}
		}
	}
}
