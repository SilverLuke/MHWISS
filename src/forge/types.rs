use std::rc::Rc;
use crate::forge::skill::{Skill, Decoration, Charm, SetSkill};
use std::cell::RefCell;
use std::collections::HashMap;
use crate::forge::armor::{Armor, Set};
use std::slice::Iter;
use crate::forge::types::Element::{Fire, Water, Thunder, Ice, Dragon, Poison, Sleep, Paralysis, Blast, Stun};
use crate::forge::types::ArmorClass::{Head, Chest, Arms, Waist, Legs};
use crate::forge::types::WeaponClass::{Bow, ChargeBlade, DualBlade, GreatSword, Gunlance, Hammer, HeavyBowgun, HuntingHorn, InsectGlaive, Lance, LightBowgun, Longsword, SwitchAxe, SwordAndShield};
use crate::forge::types::Gender::{Male, Female, All};
// use crate::forge::types::Elderseal::{High, Medium, Low};
use crate::forge::weapon::Weapon;

pub type ID = u16;
pub type Skills = RefCell<HashMap<ID, Rc<Skill>>>;
pub type SetSkills = RefCell<HashMap<ID, Rc<SetSkill>>>;
pub type Level = u8;
pub type SkillsLev = Vec<(Rc<Skill>, Level)>;
pub type Armors = RefCell<HashMap<ID, Rc<Armor>>>;
pub type Weapons = RefCell<HashMap<ID, Rc<Weapon>>>;
pub type Sets = RefCell<HashMap<ID, Rc<Set>>>;
pub type Decorations = RefCell<HashMap<ID, Rc<Decoration>>>;
pub type Charms = RefCell<HashMap<ID, Rc<Charm>>>;

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

	pub fn to_string(&self) -> String {
		match self {
			Element::Fire => "fire".to_string(),
			Element::Water => "water".to_string(),
			Element::Thunder => "thunder".to_string(),
			Element::Ice => "ice".to_string(),
			Element::Dragon => "dragon".to_string(),
			Element::Poison => "poison".to_string(),
			Element::Sleep => "sleep".to_string(),
			Element::Paralysis => "paralysis".to_string(),
			Element::Blast => "blast".to_string(),
			Element::Stun => "stun".to_string(),
		}
	}
}

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

	pub fn to_string(&self) -> String {
		match self {
			Head => "head".to_string(),
			Chest => "chest".to_string(),
			Arms => "arms".to_string(),
			Waist => "waist".to_string(),
			Legs => "legs".to_string(),
		}
	}
}

#[derive(Copy, Clone)]
pub enum Rank {
	LOW = 0,
	HIGH,
	MASTER,
}

impl Rank {
	pub fn new(rank: String) -> Rank {
		match rank.as_ref() {
			"LR" => Rank::LOW,
			"HR" => Rank::HIGH,
			"MR" => Rank::MASTER,
			_ => panic!("error")
		}
	}
}

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

	pub fn to_string(&self) -> String {
		match self {
			Bow => "bow".to_string(),
			_ => "".to_string(),
		}
	}
}

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
			(true, false) => Male,
			(false, true) => Female,
			(true, true) => All,
		}
	}
}

pub fn elder_seal(lev: Option<String>) -> u8 {
	if lev.is_none() {
		0
	} else {
		match lev.unwrap().as_str() {
			"low" => 1,
			"medium" => 2,
			"high" => 3,
			_ => panic!("Elderseal parsing error")
		}
	}
}
