
use std::collections::HashMap;
use crate::database;
use crate::forge;
use std::rc::Rc;

pub struct Forge {
    db: database::db::DB,
    pub sets: HashMap<u16, forge::armor::Set>,  // Len 343
    pub armors: HashMap<u16, Rc<forge::armor::Armor>>,
    pub skills: HashMap<u16, Rc<forge::skill::Skill>>,  // Len 168
}

impl Forge {
    pub fn new() -> Self {
        Forge {
            db: database::db::DB::new(),
            sets: HashMap::with_capacity(343),
            armors: Default::default(),
            skills: HashMap::with_capacity(168),
        }
    }

    pub fn load_all(&mut self, lang: &str) {
        self.db.load_skills(lang, &mut self.skills);
        self.db.load_armors(lang, &mut self.armors, &self.skills);
        self.db.load_set(lang, &mut self.sets, &self.armors);
        //self.skills.sort_by(|a, b| { a.name.cmp(&b.name)});
    }
}