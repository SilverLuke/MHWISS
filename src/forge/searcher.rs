use std::collections::HashMap;

enum Sign {
    GE,
    EQ
}

pub struct Searcher {
    skills: HashMap<u16, u8>,
}

impl Searcher {
    pub fn new() -> Self {
        Searcher{
            skills: Default::default(),
        }
    }

    pub fn add_skill(&mut self, id: u16, lev:u8 ) {  // Add Sign
        self.skills.insert(id, lev);
    }
}