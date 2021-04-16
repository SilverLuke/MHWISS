use std::{
    fmt,
    sync::Arc,
    cell::RefCell,
    collections::{HashMap, hash_map::Entry},
};
use crate::datatypes::{
    ID, Level, MAX_SLOTS,
    types::{ArmorClass, Gender, Rank},
    skill::{Skill, SetSkill, HasSkills, SkillLevel, SkillsLevel},
    decoration::HasDecorations,
};

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

impl HasDecorations for Armor {
    fn get_slots(&self) -> [u8; MAX_SLOTS] {
        self.slots
    }

    fn get_skills(&self) -> Box<dyn Iterator<Item=&SkillLevel> + '_> {
        Box::new(self.skills.get_skills())
    }
}

impl HasSkills for Armor {
    fn has_skills(&self, query: &HashMap<ID, u8>) -> bool {
        for skill in self.skills.get_skills() {
            if query.get(&skill.get_id()).is_some() {
                return true;
            }
        }
        false
    }

    fn get_skills_rank(&self, query: &HashMap<ID, u8>) -> u8 {
        let mut sum = 0;
        for skill in self.skills.get_skills() {
            if query.get(&skill.get_id()).is_some() {
                sum += skill.level;
            }
        }
        sum
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

pub struct ArmorSet {
    pub id: u16,
    pub name: String,
    pub rank: Rank,  // TODO Duplicate you cannot have a HR set with LR o MR armour
    set: [Option<Arc<Armor>>; 5],
    armorset_skill: Option<Arc<SetSkill>>,
}

impl ArmorSet {
    pub fn new(id: u16, name: String, rank: Rank, armorset_skill: Option<Arc<SetSkill>>) -> Self {
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