use std::{
	fmt,
	fmt::Formatter,
	slice::Iter,
	collections::HashMap,
	any::Any,
};
use crate::datatypes::{
	ID, Level,
	types::{
		Element::{Fire, Water, Thunder, Ice, Dragon, Poison, Sleep, Paralysis, Blast, Stun},
		ArmorClass::{Head, Chest, Arms, Waist, Legs},
		WeaponClass::{Bow, ChargeBlade, DualBlade, GreatSword, Gunlance, Hammer, HeavyBowgun, HuntingHorn, InsectGlaive, Lance, LightBowgun, Longsword, SwitchAxe, SwordAndShield},
	},
	skill::SkillLevel,
};

// Elements
pub enum Element {
	Fire = 0,
	Water,
	Thunder,
	Ice,
	Dragon,
	Poison,
	Sleep,
	Paralysis,
	Blast,
	Stun,
}

impl Element {
	pub fn iterator() -> Iter<'static, Element> {
		static ELEMENTS: [Element; 10] = [Fire, Water, Thunder, Ice, Dragon, Poison, Sleep, Paralysis, Blast, Stun];
		ELEMENTS.iter()
	}

	pub fn iter_element() -> Iter<'static, Element> {
		static ELEMENTS: [Element; 5] = [Fire, Water, Thunder, Ice, Dragon];
		ELEMENTS.iter()
	}

	pub fn iter_status() -> Iter<'static, Element> {
		static ELEMENTS: [Element; 5] = [Poison, Sleep, Paralysis, Blast, Stun];
		ELEMENTS.iter()
	}

	pub fn new(element: String) -> Element {
		match element.as_ref() {
			"Fire" => Element::Fire,
			"Water" => Element::Water,
			"Thunder" => Element::Thunder,
			"Ice" => Element::Ice,
			"Dragon" => Element::Dragon,
			"Poison" => Element::Poison,
			"Sleep" => Element::Sleep,
			"Paralysis" => Element::Paralysis,
			"Blast" => Element::Blast,
			"Stun" => Element::Stun,
			_ => panic!("error")
		}
	}
}

impl fmt::Display for Element {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Element::Fire => write!(f, "fire"),
			Element::Water => write!(f, "water"),
			Element::Thunder => write!(f, "thunder"),
			Element::Ice => write!(f, "ice"),
			Element::Dragon => write!(f, "dragon"),
			Element::Poison => write!(f, "poison"),
			Element::Sleep => write!(f, "sleep"),
			Element::Paralysis => write!(f, "paralysis"),
			Element::Blast => write!(f, "blast"),
			Element::Stun => write!(f, "stun"),
		}
	}
}

// Armor Class
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub enum ArmorClass {
	Head = 0,
	Chest = 1,
	Arms = 2,
	Waist = 3,
	Legs = 4
}

impl ArmorClass {
	pub fn iterator() -> Iter<'static, ArmorClass> {
		static ARMOR_CLASS: [ArmorClass; 5] = [Head, Chest, Arms, Waist, Legs];
		ARMOR_CLASS.iter()
	}

	pub fn new(armor_class: String) -> ArmorClass {
		match armor_class.as_ref() {
			"head" => Head,
			"chest" => Chest,
			"arms" => Arms,
			"waist" => Waist,
			"legs" => Legs,
			_ => panic!("error")
		}
	}
}

impl fmt::Display for ArmorClass {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Head => write!(f, "head"),
			Chest => write!(f, "chest"),
			Arms => write!(f, "arms"),
			Waist => write!(f, "waist"),
			Legs => write!(f, "legs"),
		}
	}
}

// ArmorSet rank level
#[derive(Copy, Clone)]
pub enum ArmorRank {
	Low = 0,
	High,
	Master,
}

impl ArmorRank {
	pub fn new(rank: String) -> ArmorRank {
		match rank.as_ref() {
			"LR" => ArmorRank::Low,
			"HR" => ArmorRank::High,
			"MR" => ArmorRank::Master,
			_ => panic!("error")
		}
	}
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
pub enum WeaponClass {
	Bow = 0,
	ChargeBlade,
	DualBlade,
	GreatSword,
	Gunlance,
	Hammer,
	HeavyBowgun,
	HuntingHorn,
	InsectGlaive,
	Lance,
	LightBowgun,
	Longsword,
	SwitchAxe,
	SwordAndShield
}

impl WeaponClass {
	pub fn iterator() -> Iter<'static, WeaponClass> {
		static WEAPON_CLASS: [WeaponClass; 14] = [Bow, ChargeBlade, DualBlade, GreatSword, Gunlance, Hammer, HeavyBowgun, HuntingHorn, InsectGlaive, Lance, LightBowgun, Longsword, SwitchAxe, SwordAndShield];
		WEAPON_CLASS.iter()
	}

	pub fn new(weapon_class: String) -> WeaponClass {
		match weapon_class.as_ref() {
			"bow" => Bow,
			"charge-blade" => ChargeBlade,
			"dual-blades" => DualBlade,
			"great-sword" => GreatSword,
			"gunlance" => Gunlance,
			"hammer" => Hammer,
			"heavy-bowgun" => HeavyBowgun,
			"hunting-horn" => HuntingHorn,
			"insect-glaive" => InsectGlaive,
			"lance" => Lance,
			"light-bowgun" => LightBowgun,
			"long-sword" => Longsword,
			"switch-axe" => SwitchAxe,
			"sword-and-shield" => SwordAndShield,
			_ => panic!("error")
		}
	}
}

impl fmt::Display for WeaponClass {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		match self {
			Bow => write!(f, "Bow"),
			ChargeBlade => write!(f, "ChargeBlade"),
			DualBlade => write!(f, "DualBlade"),
			GreatSword => write!(f, "GreatSword"),
			Gunlance => write!(f, "Gunlance"),
			Hammer => write!(f, "Hammer"),
			HeavyBowgun => write!(f, "HeavyBowgun"),
			HuntingHorn => write!(f, "HuntingHorn"),
			InsectGlaive => write!(f, "InsectGlaive"),
			Lance => write!(f, "Lance"),
			LightBowgun => write!(f, "LightBowgun"),
			Longsword => write!(f, "Longsword"),
			SwitchAxe => write!(f, "SwitchAxe"),
			SwordAndShield => write!(f, "SwordAndShield"),
		}
	}
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

pub trait Item : Any {
	fn has_skills(&self, query: &HashMap<ID, Level>) -> bool;
	fn get_skills_chained(&self, chained: &mut HashMap<ID, Level>);
	fn get_skills_hash(&self) -> HashMap<ID, Level>;
	fn get_skills_iter(&self) -> Box<dyn Iterator<Item=&SkillLevel> + '_>;
	fn get_slots(&self) -> Option<Vec<u8>>;
}

pub trait Decorable {}

pub trait Wearable {}