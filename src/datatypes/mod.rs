use std::collections::HashMap;
use std::sync::Arc;

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
pub mod equipment;

pub type ID = u16;
pub type Level = u8;

pub type Skills = HashMap<ID, Arc<Skill>>;
pub type SetSkills = HashMap<ID, Arc<SetSkill>>;
pub type Armors = HashMap<ID, Arc<Armor>>;
pub type Weapons = HashMap<ID, Arc<Weapon>>;
pub type Sets = HashMap<ID, Arc<ArmorSet>>;
pub type Decorations = HashMap<ID, Arc<Decoration>>;
pub type Charms = HashMap<ID, Arc<Charm>>;

pub const MAX_SLOTS: usize = 3;

pub const SHARPNESS_LEVELS: usize = 7;
