use std::collections::HashMap;
use std::cell::{RefCell, RefMut};

use crate::forge::{
    armor::Armor,
    skill::{Charm, Decoration, Skill},
    forge::Forge,
};
use std::rc::Rc;
use itertools::Itertools;
use std::fmt;

enum Sign {
    GE,
    EQ
}

pub struct Searcher {
    forge: Rc<Forge>,
    skills_req:  RefCell<HashMap<u16, (Rc<Skill>, u8)>>,
    armours:     RefCell<HashMap<u16, (Rc<Armor>, u8)>>,
    charms:      RefCell<HashMap<u16, (Rc<Charm>, u8)>>,
    decorations: RefCell<HashMap<u16, (Rc<Decoration>, u8)>>,

}

impl fmt::Display for Searcher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut str;
        str = format!("###    ARMORS   ###\n");
        for (_, (armor, rank)) in self.armours.borrow().iter().sorted_by(|(_, (_, a)), (_,(_, b))| {b.cmp(&a)})  {
            str = format!("{} {} : {}\n", str, rank, armor);
        }
        str = format!("{}###    CHARMS   ###\n", str);
        for (_, (charm, rank)) in self.charms.borrow().iter().sorted_by(|(_, (_, a)), (_,(_, b))| {b.cmp(&a)})  {
            str = format!("{} {} : {}\n", str, rank,  charm);
        }
        str = format!("{}###    DECORATIONS   ###\n", str);
        for (_, (deco, rank)) in self.decorations.borrow().iter().sorted_by(|(_, (_, a)), (_,(_, b))| {b.cmp(&a)})  {
            str = format!("{} {} : {}\n", str, rank, deco);
        }
        write!(f, "{}##################\n", str)
    }
}

impl Searcher {
    pub fn new(forge: Rc<Forge>) -> Self {
        Searcher{
            skills_req:  Default::default(),
            armours:     Default::default(),
            charms:      Default::default(),
            decorations: Default::default(),
            forge,
        }
    }

    pub fn add_skill_requirement(&self, skill: Rc<Skill>, lev: u8) {  // Add Sign
        if lev == 0 {
            self.skills_req.borrow_mut().remove(&skill.id);
        } else {
            self.skills_req.borrow_mut().insert(skill.id, (skill, lev));
        }
    }

    pub fn show_requirements(&self) {
        println!("Requirements: ");
        for (_id, (skill, lev)) in self.skills_req.borrow().iter() {
            println!("Skill: {} lev: {}", skill.name, lev);
        }
    }

    fn filter(&self) {
        self.armours.replace(self.forge.get_armors_filtered(&self.skills_req));
        self.charms.replace(self.forge.get_charms_filtered(&self.skills_req));
        self.decorations.replace(self.forge.get_decorations_filtered(&self.skills_req));
        println!("ARMORS: {} CHARMS: {}, DECORATIONS: {}", self.armours.borrow().len(), self.charms.borrow().len(), self.decorations.borrow().len());
    }

    fn clean(&self) {
        self.armours.replace(Default::default());
        self.charms.replace(Default::default());
        self.decorations.replace(Default::default());
    }

    pub fn calculate(&self) {
        self.clean();
        self.filter();
    }

}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::forge;

    #[test]
    fn filtering() {
        let forge = Rc::new(forge::forge::Forge::new());
        forge.load_all("it");
        let searcher = forge::searcher::Searcher::new(Rc::clone(&forge));

        searcher.add_skill_requirement(forge.get_skill("Occhio critico").unwrap(), 3);
        searcher.add_skill_requirement(forge.get_skill("Bombardiere").unwrap(), 3);
        searcher.add_skill_requirement(forge.get_skill("Critico elementale").unwrap(), 1);

        searcher.calculate();
        println!("{}", searcher);
        assert_eq!("a", "a");
    }
}