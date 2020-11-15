use std::rc::Rc;
use crate::forge::skill::{Skill, Decoration, Charm};
use std::cell::RefCell;
use std::collections::HashMap;
use crate::forge::armor::{Armor, Set};

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
	Paralysing,
	Blast,
	Stun,
}