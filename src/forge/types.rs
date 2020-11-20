use std::rc::Rc;
use crate::forge::skill::{Skill, Decoration, Charm};
use std::cell::RefCell;
use std::collections::HashMap;
use crate::forge::armor::{Armor, Set};
use std::slice::Iter;
use crate::forge::types::Element::{Fire, Water, Thunder, Ice, Dragon, Poison, Sleep, Paralysis, Blast, Stun};
use crate::forge::types::ArmorClass::{Head, Chest, Arms, Waist, Legs};
use crate::forge::types::WeaponClass::{Bow, ChargeBlade, DualBlade, Greatsword, Gunlance, Hammer, HeavyBowgun, HuntingHorn, InsectGlaive, Lance, LightBowgun, Longsword, SwitchAxe, SwordAndShield};
use crate::forge::types::Gender::{Male, Female, All};

pub type ID = u16;
pub type Skills = RefCell<HashMap<ID, Rc<Skill>>>;
pub type SkillsLev = Vec<(Rc<Skill>, u8)>;
pub type Armors = RefCell<HashMap<ID, Rc<Armor>>>;
pub type Sets = RefCell<HashMap<ID, Set>>;
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
			"fire" => Element::Fire,
			"water" => Element::Water,
			"thunder" => Element::Thunder,
			"ice" => Element::Ice,
			"dragon" => Element::Dragon,
			"poison" => Element::Poison,
			"sleep" => Element::Sleep,
			"paralysis" => Element::Paralysis,
			"blast" => Element::Blast,
			"stun" => Element::Stun,
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
	Greatsword,
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
		static WEAPON_CLASS: [WeaponClass; 14] = [Bow, ChargeBlade, DualBlade, Greatsword, Gunlance, Hammer, HeavyBowgun, HuntingHorn, InsectGlaive, Lance, LightBowgun, Longsword, SwitchAxe, SwordAndShield];
		WEAPON_CLASS.iter()
	}

	pub fn new(weapon_class: String) -> WeaponClass {
		match weapon_class.as_ref() {
			"bow" => Bow,
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

pub enum Gender{
	Male,
	Female,
	All,
}

impl Gender {
	fn new(i: u8) -> Self {
		match i {
			1 => Male,
			10 => Female,
			11 => All,
			_ => panic!("Unknow gender")
		}
	}
}