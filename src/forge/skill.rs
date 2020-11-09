use std::rc::Rc;

pub struct Skill {
    pub id: u16,
    pub name: String,
    pub description: String,
    pub dependency: Option<u8>,
    pub max_level: u8,
}

impl Skill{
    pub fn new(id: u16, name: String, description: String, max_level: u8) -> Self {
        Skill { id, name, description, max_level, dependency: None }
    }
}

pub struct Charm {
    pub id: u16,
    pub name: String,
    pub skills: Vec<(Rc<Skill>, u8)>,
}

impl Charm{
    pub fn new(id: u16, name: String) -> Self {
        Charm { id, name, skills: Vec::with_capacity(1) }
    }

    pub fn add_skill(&mut self, skill: &Rc<Skill>, level: u8) {
        self.skills.push((Rc::clone(skill), level));
    }
}

pub struct Decoration {
    pub id: u16,
    pub name: String,
    pub skills: [Option<(Rc<Skill>, u8)>; 2],
}

impl Decoration {
    pub fn new(id: u16, name: String, skills: [Option<(Rc<Skill>, u8)>; 2]) -> Self {
        Decoration { id, name, skills }
    }
}
