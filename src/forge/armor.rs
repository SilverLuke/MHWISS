use crate::forge::skill::Skill;
use std::rc::Rc;
use std::collections::HashMap;
use std::fmt;
use crate::forge::types;
use crate::forge::types::{ArmorClass, Rank};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Armor {
    pub id: u16,
    pub name: String,
    pub class: ArmorClass,
    pub skills: types::SkillsLev,
    decorations: [u8; 3],
    defense_base: u8,
    defense_max: u16,
    defense_arg: u16,
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
    pub fn new(id: u16, name: String, class: ArmorClass, decorations: [u8; 3], defense_base: u8, defense_max: u16, defense_arg: u16) -> Self {
        Armor { id, name, class, skills: Vec::new(), decorations, defense_base, defense_max, defense_arg }
    }

    pub fn add_skill(&mut self, skill: &Rc<Skill>, level: u8) {
        self.skills.push((Rc::clone(skill), level));
    }

    pub fn get_skills_rank(&self, query: &HashMap<Rc<Skill>, u8>) -> Option<u8> {
        let mut rank: u8 = 0;
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
    set: [Option<Rc<Armor>>; 5],
    pub rank: Rank,
}

impl Set {
    pub fn new(id: u16, name: String, rank: Rank) -> Self {
        Set { id, name, set: [None, None, None, None, None], rank }
    }

    pub fn add_element(&mut self, armor_type: ArmorClass, armor: &Rc<Armor>) {
        let i = armor_type as usize;
        if self.set[i].is_some() {
            panic!("Element of a set already full");
        }
        else {
            self.set[i] = Some(Rc::clone(armor));
        }
    }
}