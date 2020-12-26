use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{RefCell};

use crate::forge::{
    types::{Armors, Skills, Sets, Charms, Decorations, SetSkills, Weapons, ID},
    skill::{Skill, Charm, Decoration},
    armor::Armor,
};
use crate::database;
use crate::searcher::container::HasSkills;

pub struct Forge {
    pub skills: Skills,  // Len 168
    pub set_skills: SetSkills,
    pub armors: Armors,
    pub sets: Sets,  // Len 343
    pub decorations: Decorations,
    pub charms: Charms,
    pub weapons: Weapons,
}

impl Forge {
    pub fn new() -> Self {
        Forge {
            skills: Default::default(),
            set_skills:Default::default(),
            armors: Default::default(),
            sets: Default::default(),
            decorations: Default::default(),
            charms: Default::default(),
            weapons: Default::default(),
        }
    }

    pub fn get_skill(&self, name: &str) -> Option<Rc<Skill>> {
        for (_, skill) in self.skills.iter() {
            if skill.name == name {
                return Some(Rc::clone(skill));
            }
        }
        None
    }

    pub fn get_armors_filtered(&self, skills_req: &RefCell<HashMap<ID, u8>>) -> Vec<Rc<Armor>> {
        let mut ret: Vec<Rc<Armor>>  = Default::default();
            for (_id, armor) in self.armors.iter() {
                if armor.has_skills(&skills_req) {
                    ret.push(Rc::clone(armor));
                }
            }
        ret.shrink_to_fit();
        ret
    }

    pub fn get_charms_filtered(&self, skills_req: &RefCell<HashMap<ID, u8>>) -> Vec<(Rc<Charm>, u8)> {
        let mut ret  = vec![];

        for (_id, charm) in self.charms.iter() {
            if charm.has_skills(skills_req) {
                let val = 1u8;  // valutate_charm(charm);
                ret.push((Rc::clone(charm), val));
            }
        }
        ret
    }

    pub fn get_decorations_filtered(&self, skills_req: &RefCell<HashMap<ID, u8>>) -> Vec<(Rc<Decoration>, u8)> {
        let mut ret  = vec![];
        for (_id, deco) in self.decorations.iter() {
            if deco.has_skills(skills_req) {
                let val = 1u8;  // valutate_charm(charm);
                ret.push((Rc::clone(deco), val));
            }
        }
        ret
    }

    pub fn load_all(&mut self, lang: &str) {
        let db = database::db::DB::new();
        db.set_lang(lang.to_string());
        db.load_skills(&mut self.skills);
        db.load_setskills(&mut self.set_skills, &self.skills);
        db.load_armors(&mut self.armors, &self.skills, &self.set_skills);
        db.load_sets(&mut self.sets, &self.armors, &self.set_skills);
        db.load_charms(&mut self.charms, &self.skills);
        db.load_decorations(&mut self.decorations, &self.skills);
        db.load_weapons(&mut self.weapons, &self.skills, &self.set_skills);
    }

    pub fn print_stat(&self) {
        println!("Loaded {} skills", self.skills.len());
        println!("Loaded {} armorset skills", self.skills.len());
        println!("Loaded {} armors", self.armors.len());
        println!("Loaded {} sets", self.sets.len());
        println!("Loaded {} charms", self.charms.len());
        println!("Loaded {} decorations", self.decorations.len());
        println!("Loaded {} weapons", self.weapons.len());
    }

}