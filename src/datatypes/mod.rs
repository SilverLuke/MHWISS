use std::collections::HashMap;
use std::rc::Rc;

use crate::datatypes::{
	armor::{Armor, ArmorSet},
	charm::Charm,
	decoration::Decoration,
	skill::{SetSkill, Skill},
	weapon::Weapon,
};

pub mod forge;
pub mod skill;
pub mod armor;
pub mod types;
pub mod weapon;
pub mod decoration;
pub mod charm;
pub mod tool;

pub type ID = u16;
pub type Skills = HashMap<ID, Rc<Skill>>;
pub type SetSkills = HashMap<ID, Rc<SetSkill>>;
pub type Armors = HashMap<ID, Rc<Armor>>;
pub type Weapons = HashMap<ID, Rc<Weapon>>;
pub type Sets = HashMap<ID, Rc<ArmorSet>>;
pub type Decorations = HashMap<ID, Rc<Decoration>>;
pub type Charms = HashMap<ID, Rc<Charm>>;

pub type Level = u8;
pub type SkillLev = (Rc<Skill>, Level);
pub type SkillsLev = Vec<SkillLev>;

pub const MAX_SLOTS : usize = 3;
