use crate::forge::skill::{Skill, SetSkill};
use std::rc::Rc;
use std::collections::HashMap;
use std::fmt;
use crate::forge::types;
use crate::forge::types::{ArmorClass, Rank, SkillsLev, Gender, ID};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Armor {
    pub id: u16,
    pub name: String,
    pub class: ArmorClass,
    pub skills: types::SkillsLev,  // Set skills go here
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
        write!(f, "name: {}[{}] skills: {}", self.name, self.id, str)
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

    pub fn get_skills_rank(&self, query: &HashMap<Rc<Skill>, u8>) -> Option<u8> {
        let mut rank: u8 = 0;  // TODO use Option<u8>
        for (skill, lev) in self.skills.iter() {
            if query.get(skill).is_some() {
                rank += lev;
            }
        }
        if rank == 0 {
            return None;
        }
        Some(rank)
    }
}


pub struct Set {
    pub id: u16,
    pub name: String,
    pub rank: Rank,  // Duplicate you cannot have a HR set with LR o MR armour TO BE FIXED
    set: [Option<Rc<Armor>>; 5],
    armorset_skill: Option<Rc<SetSkill>>,
}

impl Set {
    pub fn new(id: u16, name: String, rank: Rank, armorset_skill: Option<Rc<SetSkill>>) -> Self {
        Set { id, name, rank, set: [None, None, None, None, None], armorset_skill}
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