use std::collections::HashMap;
use crate::database;
use crate::forge;
use std::rc::Rc;
use std::cell::{RefCell};

pub struct Forge {
    db: database::db::DB,
    pub skills: RefCell<HashMap<u16, Rc<forge::skill::Skill>>>,  // Len 168
    pub armors: RefCell<HashMap<u16, Rc<forge::armor::Armor>>>,
    pub sets: RefCell<HashMap<u16, forge::armor::Set>>,  // Len 343
    pub decorations: RefCell<HashMap<u16, Rc<forge::skill::Decoration>>>,
    pub charms: RefCell<HashMap<u16, Rc<forge::skill::Charm>>>,
}

impl Forge {
    pub fn new() -> Self {
        Forge {
            db: database::db::DB::new(),
            skills: RefCell::new(HashMap::with_capacity(168)),
            armors: RefCell::new(Default::default()),
            sets: RefCell::new(HashMap::with_capacity(343)),
            decorations: RefCell::new(Default::default()),
            charms: RefCell::new(Default::default())
        }
    }

    pub fn load_all(&self, lang: &str) {
        self.db.set_lang(lang.to_string());
        self.db.load_skills(&self.skills);
        println!("Loaded {} skills", self.skills.borrow().len());
        self.db.load_armors( &self.armors, &self.skills);
        println!("Loaded {} armors", self.armors.borrow().len());
        self.db.load_set(&self.sets, &self.armors);
        println!("Loaded {} sets", self.sets.borrow().len());
        self.db.load_charms(&self.charms, &self.skills);
        println!("Loaded {} charms", self.charms.borrow().len());
        self.db.load_decorations(&self.decorations, &self.skills);
        println!("Loaded {} decorations", self.decorations.borrow().len());
    }
}