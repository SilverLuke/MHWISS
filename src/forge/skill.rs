use std::rc::Rc;
use std::fmt;
use std::collections::HashMap;

pub struct Skill {
    pub id: u16,
    pub name: String,
    pub description: String,
    pub dependency: Option<u8>,  // TODO
    pub max_level: u8,
}
impl fmt::Display for Skill {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}[{}]", self.name, self.id)
    }
}

impl Skill {
    pub fn new(id: u16, name: String, description: String, max_level: u8) -> Self {
        Skill { id, name, description, max_level, dependency: None }
    }
}

pub struct Charm {
    pub id: u16,
    pub name: String,
    pub skills: Vec<(Rc<Skill>, u8)>,
}
impl fmt::Display for Charm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::new();
        for (skill, lev) in self.skills.iter() {
            str = format!("{} <{}, {}>", str, *skill, lev);
        }
        write!(f, "{}[{}] Skill: {}", self.name, self.id, str)
    }
}

impl Charm{
    pub fn new(id: u16, name: String) -> Self {
        Charm { id, name, skills: Vec::with_capacity(1) }
    }

    pub fn add_skill(&mut self, skill: &Rc<Skill>, level: u8) {
        self.skills.push((Rc::clone(skill), level));
    }

    pub fn get_skills_rank(&self, query: &HashMap<u16, (Rc<Skill>, u8)>) -> Option<u8> {
        let mut rank: u8 = 0;
        for (skill, lev) in self.skills.iter() {
            if query.get(&skill.id).is_some() {
                rank += lev;
            }
        }
        if rank == 0 {
            return None;
        }
        Some(rank)
    }
}

pub struct Decoration {
    pub id: u16,
    pub name: String,
    pub size: u8,
    pub skills: Vec<(Rc<Skill>, u8)>,
}
impl fmt::Display for Decoration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str = String::new();
        for (skill, lev) in self.skills.iter() {
            str = format!("{} <{}, {}>", str, *skill, lev);
        }
        write!(f, "{}[{}] Skill: {}", self.name, self.id, str)
    }
}
impl Decoration {
    pub fn new(id: u16, name: String, size: u8, skills: Vec<(Rc<Skill>, u8)>) -> Self {
        Decoration { id, name, size, skills }
    }

    pub fn get_skills_rank(&self, query: &HashMap<u16, (Rc<Skill>, u8)>) -> Option<u8> {
        let mut rank: u8 = 0;
        for (skill, lev) in self.skills.iter() {
            if query.get(&skill.id).is_some() {
                rank += lev;
            }
        }
        if rank == 0 {
            return None;
        }
        Some(rank)
    }
}
