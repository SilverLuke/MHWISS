use crate::forge::skill::Skill;
use std::rc::Rc;


#[derive(Debug)]
pub enum ArmorType {
    HEAD = 0,
    CHEST = 1,
    ARMS = 2,
    WAIST = 3,
    LEGS = 4
}

pub fn tr_armor_type (armor_type: String ) -> ArmorType {
    match armor_type.as_ref() {
        "head"  => ArmorType::HEAD,
        "chest" => ArmorType::CHEST,
        "arms"  => ArmorType::ARMS,
        "waist" => ArmorType::WAIST,
        "legs"  => ArmorType::LEGS,
        _ => panic!("error")
    }
}

pub enum Rank {
    LOW = 0,
    HIGH = 1,
    MASTER = 2,
}

pub fn tr_rank (rank: String ) -> Rank {
    match rank.as_ref() {
        "LR"  => Rank::LOW,
        "HR" => Rank::HIGH,
        "MR"  => Rank::MASTER,
        _ => panic!("error")
    }
}

pub struct Armor {
    pub id: u16,
    pub name: String,
    skills: Vec<(Rc<Skill>, u8)>,
    decorations: [u8; 3],
    defense_base: u8,
    defense_max: u16,
    defense_arg: u16,
}

impl Armor {
    pub fn new(id: u16, name: String, decorations: [u8; 3], defense_base: u8, defense_max: u16, defense_arg: u16) -> Self {
        Armor { id, name, skills: Vec::new(), decorations, defense_base, defense_max, defense_arg }
    }

    pub fn add_skill(&mut self, skill: &Rc<Skill>, level: u8) {
        self.skills.push((Rc::clone(skill), level));
    }
}

pub struct Set {
    pub id: u16,
    pub name: String,
    set: [Option<Rc<Armor>>; 5],
    pub rank: self::Rank,
}

impl Set {
    pub fn new(id: u16, name: String, rank: Rank) -> Self {
        Set { id, name, set: [None, None, None, None, None], rank }
    }

    pub fn add_element(&mut self, armor_type: ArmorType, armor: &Rc<Armor>) {
        let i = armor_type as usize;
        if self.set[i].is_some() {
            panic!("Element of a set already full");
        }
        else {
            self.set[i] = Some(Rc::clone(armor));
        }
    }

    pub fn rank_index (&self) -> usize {
        match self.rank {
            Rank::LOW => 0,
            Rank::HIGH => 1,
            Rank::MASTER => 2
        }
    }
}