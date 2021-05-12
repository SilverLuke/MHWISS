use std::{
    collections::HashMap,
    slice::Iter,
};
use strum::{Display, EnumIter, EnumString, EnumCount};

use crate::datatypes::{
    ID, Level,
    skill::SkillLevel,
    types::{
        Element::{Blast, Dragon, Fire, Ice, Paralysis, Poison, Sleep, Stun, Thunder, Water},
	},
};

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

	pub fn iter_status() -> Iter<'static, Element> {
		static ELEMENTS: [Element; 5] = [Poison, Sleep, Paralysis, Blast, Stun];
		ELEMENTS.iter()
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

pub trait Item {
	fn has_skills(&self, query: &HashMap<ID, Level>) -> bool;
	fn get_skills_hash(&self) -> HashMap<ID, Level>;
	fn get_skills_chained(&self, chained: &mut HashMap<ID, Level>);
	fn get_skills_iter(&self) -> Box<dyn Iterator<Item=&SkillLevel> + '_>;
	fn get_slots(&self) -> Option<Vec<u8>>;
}

pub trait Decorable {
	fn get_slots(&self) -> Vec<u8>;
}

pub trait Wearable {}