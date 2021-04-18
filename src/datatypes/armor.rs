use std::{
    fmt,
    sync::Arc,
    cell::RefCell,
    collections::{HashMap, hash_map::Entry},
};
use crate::datatypes::{
    ID, Level, MAX_SLOTS,
    types::{ArmorClass, Gender, ArmorRank},
    skill::{Skill, SetSkill, SkillLevel, SkillsLevel},
};
use crate::datatypes::types::Item;
use rusqlite::OpenFlags;
use std::ops::Not;

pub struct Armor {
    pub id: u16,
    pub name: String,
    pub class: ArmorClass,
    pub skills: SkillsLevel,  // Set skills go here
    pub setskill: Option<Arc<SetSkill>>,
    pub gender: Gender,
    pub slots: [u8; MAX_SLOTS],
    pub defence: [u8; 3],
    pub elements : [i8; 5],
}

impl Armor {
    pub fn new(id: u16, name: String, class: ArmorClass, gender: Gender, slots: [u8; MAX_SLOTS], defence: [u8; 3], elements: [i8; 5]) -> Self {
        Armor { id, name, class, skills: SkillsLevel::new(), setskill: None, gender, slots, defence, elements }
    }

    pub fn add_skill(&mut self, skill: &Arc<Skill>, level: u8) {
        self.skills.update_or_append(SkillLevel::new(Arc::clone(skill), level));
    }

    pub fn add_setskill(&mut self, setskill: &Arc<SetSkill>) {
        self.setskill = Some(Arc::clone(setskill));
    }
}

impl fmt::Display for Armor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::new();
        for skill in self.skills.get_skills() {
            str = format!("{} <{}, {}>", str, skill.skill, skill.level);
        }
        write!(f, "{0: <45}|{1: <50}", format!("{} [{}] {}", self.name, self.id, self.defence[2]), str)
    }
}

impl PartialEq for Armor {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Armor {}

impl Item for Armor {
    fn has_skills(&self, query: &HashMap<ID, Level>) -> bool {
        self.skills.contains_hash(query)
    }

    fn get_skills_chained(&self, chained: &mut HashMap<ID, Level>) {
        self.skills.put_in(chained);
    }

    fn get_skills_hash(&self) -> HashMap<ID, Level> {
        self.skills.as_hash()
    }

    fn get_skills_iter(&self) -> Box<dyn Iterator<Item=&SkillLevel> + '_> {
        self.skills.get_skills()
    }

    fn get_slots(&self) -> Option<Vec<u8>> {
        Some(Vec::from(self.slots))
    }
}


pub struct ArmorSet {
    pub id: u16,
    pub name: String,
    pub rank: ArmorRank,
    set: [Option<Arc<Armor>>; 5],
    armorset_skill: Option<Arc<SetSkill>>,
}

impl ArmorSet {
    pub fn new(id: u16, name: String, rank: ArmorRank, armorset_skill: Option<Arc<SetSkill>>) -> Self {
        ArmorSet { id, name, rank, set: [None, None, None, None, None], armorset_skill}
    }

    pub fn add_piece(&mut self, armor: &Arc<Armor>) {
        let i = armor.class as usize;
        if self.set[i].is_some() {
            panic!("Element of a set already full");
        }
        else {
            self.set[i] = Some(Arc::clone(armor));
        }
    }
}