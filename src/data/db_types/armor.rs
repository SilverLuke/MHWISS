use std::{
    fmt,
    sync::Arc,
    hash::{Hash, Hasher}
};
use strum::EnumCount;
use crate::data::db_types::{
    ID,
    MAX_SLOTS,
    ArmorClass,
    ArmorRank,
    Gender,
	Item,
    Element,
    skill::{SetSkill, Skill, SkillLevel, SkillsLevel},
};


pub struct Armor {
    pub id: ID,
    pub name: String,
    pub class: ArmorClass,
    pub rank: ArmorRank,
    pub skills: SkillsLevel,
    pub set_skill: Option<Arc<SetSkill>>,  // Set skills go here
    pub gender: Gender,
    pub slots: [u8; MAX_SLOTS],
    pub defence: [u8; 3],
    pub elements : [i8; Element::len()],
}

impl Armor {
    pub fn new(id: u16, name: String, class: ArmorClass, rank: ArmorRank, gender: Gender, slots: [u8; MAX_SLOTS], defence: [u8; 3], elements: [i8; 5]) -> Self {
        Armor { id, name, class, rank, skills: SkillsLevel::new(), set_skill: None, gender, slots, defence, elements }
    }

    pub fn add_skill(&mut self, skill: &Arc<Skill>, level: u8) {
        self.skills.insert(SkillLevel::new(Arc::clone(skill), level));
    }

    pub fn add_setskill(&mut self, setskill: &Arc<SetSkill>) {
        self.set_skill = Some(Arc::clone(setskill));
    }
}

impl fmt::Display for Armor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0: <45}|{1: <45}", format!("{} [{}] {}", self.name, self.id, self.defence[2]), self.skills.to_string())
    }
}

impl Item for Armor {
    fn get_skills(&self) -> SkillsLevel {
        self.skills.clone()
    }
    fn get_slots(&self) -> Vec<u8> {
        Vec::from(self.slots)
    }
}

impl PartialEq for Armor {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Armor {}

impl Hash for Armor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[allow(dead_code)]
pub struct ArmorSet {
    pub id: u16,
    pub name: String,
    pub rank: ArmorRank,
    pub armors: [Option<Arc<Armor>>; ArmorClass::COUNT],
    armorset_skill: Option<Arc<SetSkill>>,
}

impl ArmorSet {
    pub fn new(id: u16, name: String, rank: ArmorRank, armorset_skill: Option<Arc<SetSkill>>) -> Self {
        ArmorSet { id, name, rank, armors: [None, None, None, None, None], armorset_skill}
    }

    pub fn add_armor(&mut self, armor: &Arc<Armor>) {
        let i = armor.class as usize;
        if self.armors[i].is_some() {
            panic!("Element of a set already full");
        }
        else {
            self.armors[i] = Some(Arc::clone(armor));
        }
    }

    pub fn get_armor(&self, id: ArmorClass) -> &Option<Arc<Armor>> {
        self.armors.get(id as usize).unwrap()
    }
}

impl PartialEq for ArmorSet {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for ArmorSet {}

impl Hash for ArmorSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}