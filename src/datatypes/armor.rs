use std::rc::Rc;
use std::collections::HashMap;
use std::fmt;
use crate::datatypes::*;
use std::cell::RefCell;
use crate::datatypes::types::{ArmorClass, Gender, Rank};
use crate::datatypes::decoration::HasDecorations;
use crate::datatypes::skill::HasSkills;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Armor {
    pub id: u16,
    pub name: String,
    pub class: ArmorClass,
    pub skills: SkillsLev,  // Set skills go here
    pub setskill: Option<Rc<SetSkill>>,
    pub gender: Gender,
    pub slots: [u8; 3],
    pub defence: [u8; 3],
    pub elements : [i8; 5],
}

impl fmt::Display for Armor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::new();
        for (skill, lev) in self.skills.iter() {
            str = format!("{} <{}, {}>", str, *skill, lev);
        }
        write!(f, "{0: <45}|{1: <50}", format!("{} [{}] {}", self.name, self.id, self.defence[2]), str)
    }
}

impl Armor {
    pub fn new(id: u16, name: String, class: ArmorClass, gender: Gender, slots: [u8; 3], defence: [u8; 3], elements: [i8; 5]) -> Self {
        Armor { id, name, class, skills: Vec::new(), setskill: None, gender, slots, defence, elements }
    }

    pub fn add_skill(&mut self, skill: &Rc<Skill>, level: u8) {  // TODO add SkillLev "object"
        self.skills.push((Rc::clone(skill), level));
    }

    pub fn add_setskill(&mut self, setskill: &Rc<SetSkill>) {
        self.setskill = Some(Rc::clone(setskill));
    }

}

impl HasDecorations for Armor {
    fn get_slots(&self) -> [u8; 3] {
        self.slots
    }

    fn get_skills(&self) -> Box<dyn Iterator<Item=&SkillLev> + '_> {
        Box::new(self.skills.iter())
    }
}

impl HasSkills for Armor {
    fn has_skills(&self, query: &RefCell<HashMap<ID, u8>>) -> bool {
        for (skill, lev) in self.skills.iter() {
            if query.borrow().get(&skill.id).is_some() {
                return true;
            }
        }
        false
    }
    fn get_skills_rank(&self, query: &RefCell<HashMap<ID, u8>>) -> Option<u8> {
        let mut sum = 0;
        for (skill, lev) in self.skills.iter() {
            if query.borrow().get(&skill.id).is_some() {
                sum += lev;
            }
        }
        if sum != 0 {
            return Some(sum);
        }
        None
    }
}


pub struct ArmorSet {
    pub id: u16,
    pub name: String,
    pub rank: Rank,  // TODO Duplicate you cannot have a HR set with LR o MR armour
    set: [Option<Rc<Armor>>; 5],
    armorset_skill: Option<Rc<SetSkill>>,
}

impl ArmorSet {
    pub fn new(id: u16, name: String, rank: Rank, armorset_skill: Option<Rc<SetSkill>>) -> Self {
        ArmorSet { id, name, rank, set: [None, None, None, None, None], armorset_skill}
    }

    pub fn add_piece(&mut self, armor: &Rc<Armor>) {
        let i = armor.class as usize;
        if self.set[i].is_some() {
            panic!("Element of a set already full");
        }
        else {
            self.set[i] = Some(Rc::clone(armor));
        }
    }
}