use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{RefCell};
use std::borrow::Borrow;

use crate::forge::types::{ID, Armors, Skills, Sets, Charms, Decorations};
use crate::database;
use crate::forge::skill::{Skill, Charm, Decoration};
use crate::forge::armor::{Armor, Set, ArmorClass};

pub struct Forge {
    pub skills: Skills,  // Len 168
    pub armors: Armors,
    pub sets: Sets,  // Len 343
    pub decorations: Decorations,
    pub charms: Charms,
}

impl Forge {
    pub fn new() -> Self {
        Forge {
            skills: RefCell::new(HashMap::with_capacity(168)),
            armors: RefCell::new(Default::default()),
            sets: RefCell::new(HashMap::with_capacity(343)),
            decorations: RefCell::new(Default::default()),
            charms: RefCell::new(Default::default())
        }
    }

    pub fn get_skill(&self, name: &str) -> Option<Rc<Skill>> {
        for (_, skill) in self.skills.borrow().iter() {
            if skill.name == name {
                return Some(Rc::clone(skill));
            }
        }
        None
    }

    pub fn get_armors_filtered(&self, skills_req: &RefCell<HashMap<Rc<Skill>, u8>>) -> Vec<(Rc<Armor>, u8)> {
        let mut ret: Vec<(Rc<Armor>, u8)>  = Default::default();
        let skills = skills_req.borrow();
            for (_id, armor) in self.armors.borrow().iter() {
                let rank = armor.get_skills_rank(&skills);
                if  rank.is_some() {
                    ret.push((Rc::clone(armor), rank.unwrap()));
                }
            }
        ret.shrink_to_fit();
        ret
    }

    pub fn get_charms_filtered(&self, skills_req: &RefCell<HashMap<Rc<Skill>, u8>>) -> HashMap<Rc<Charm>, u8> {
        let mut ret: HashMap<Rc<Charm>, u8>  = Default::default();
        let skills = skills_req.borrow();

        for (id, charm) in self.charms.borrow().iter() {
            let rank = charm.get_skills_rank(&skills);
            if  rank.is_some() {
                ret.insert(Rc::clone(charm), rank.unwrap());
            }
        }
        ret
    }

    pub fn get_decorations_filtered(&self, skills_req: &RefCell<HashMap<Rc<Skill>, u8>>) -> HashMap<Rc<Decoration>, u8> {
        let mut ret: HashMap<Rc<Decoration>, u8>  = Default::default();
        let skills = skills_req.borrow();
        for (id, deco) in self.decorations.borrow().iter() {
            let rank = deco.get_skills_rank(&skills);
            if  rank.is_some() {
                ret.insert(Rc::clone(deco), rank.unwrap());
            }
        }
        ret
    }

    pub fn load_all(&self, lang: &str) {
        let db = database::db::DB::new();
        db.set_lang(lang.to_string());
        db.load_skills(&self.skills);
        println!("Loaded {} skills", self.skills.borrow().len());
        db.load_armors( &self.armors, &self.skills);
        println!("Loaded {} armors", self.armors.borrow().len());
        db.load_set(&self.sets, &self.armors);
        println!("Loaded {} sets", self.sets.borrow().len());
        db.load_charms(&self.charms, &self.skills);
        println!("Loaded {} charms", self.charms.borrow().len());
        db.load_decorations(&self.decorations, &self.skills);
        println!("Loaded {} decorations", self.decorations.borrow().len());
    }
}