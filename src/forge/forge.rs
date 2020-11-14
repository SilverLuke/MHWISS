use std::collections::HashMap;
use crate::database;
use crate::forge;
use std::rc::Rc;
use std::cell::{RefCell};
use crate::forge::skill::{Skill, Charm, Decoration};
use crate::forge::armor::{Armor, Set};
use std::borrow::Borrow;

type ID = u16;

pub struct Forge {
    pub skills: RefCell<HashMap<ID, Rc<Skill>>>,  // Len 168
    pub armors: RefCell<HashMap<ID, Rc<Armor>>>,
    pub sets: RefCell<HashMap<ID, Set>>,  // Len 343
    pub decorations: RefCell<HashMap<ID, Rc<Decoration>>>,
    pub charms: RefCell<HashMap<ID, Rc<Charm>>>,
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

    pub fn get_armors_filtered(&self, skills_req: &RefCell<HashMap<ID, (Rc<Skill>, u8)>>) -> HashMap<ID, (Rc<Armor>, u8)> {
        let mut ret: HashMap<ID, (Rc<Armor>, u8)>  = Default::default();
        let skills = skills_req.borrow();
        for (id, armor) in self.armors.borrow().iter() {
            let rank = armor.get_skills_rank(&skills);
            if  rank.is_some() {
                ret.insert(*id, (Rc::clone(armor), rank.unwrap()));
            }
        }
        ret
    }
    pub fn get_charms_filtered(&self, skills_req: &RefCell<HashMap<ID, (Rc<Skill>, u8)>>) -> HashMap<ID, (Rc<Charm>, u8)> {
        let mut ret: HashMap<ID, (Rc<Charm>, u8)>  = Default::default();
        let skills = skills_req.borrow();
        for (id, charm) in self.charms.borrow().iter() {
            let rank = charm.get_skills_rank(&skills);
            if  rank.is_some() {
                ret.insert(*id, (Rc::clone(charm), rank.unwrap()));
            }
        }
        ret
    }
    pub fn get_decorations_filtered(&self, skills_req: &RefCell<HashMap<ID, (Rc<Skill>, u8)>>) -> HashMap<ID, (Rc<Decoration>, u8)> {
        let mut ret: HashMap<ID, (Rc<Decoration>, u8)>  = Default::default();
        let skills = skills_req.borrow();
        for (id, deco) in self.decorations.borrow().iter() {
            let rank = deco.get_skills_rank(&skills);
            if  rank.is_some() {
                ret.insert(*id, (Rc::clone(deco), rank.unwrap()));
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