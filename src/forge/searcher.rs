use std::collections::HashMap;
use std::cell::{RefCell};

enum Sign {
    GE,
    EQ
}

pub struct Searcher {
    skills: RefCell<HashMap<u16, u8>>,
}

impl Searcher {
    pub fn new() -> Self {
        Searcher{
            skills: Default::default(),
        }
    }

    pub fn add_skill(&self, id: u16, lev:u8 ) {  // Add Sign
        println!("Skill: {} lev: {}", id, lev);
        if lev == 0 {
            self.skills.borrow_mut().remove(&id);
        } else {
            self.skills.borrow_mut().insert(id, lev);
        }
    }

    pub fn print(&self) {
        println!("Requirements: ");
        for (id, lev) in self.skills.borrow().iter() {
            println!("Skill: {} lev: {}", id, lev);
        }
    }
}