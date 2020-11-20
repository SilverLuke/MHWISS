use std::collections::HashMap;
use std::cell::{RefCell};

use crate::forge::{
	armor::Armor,
	skill::{Charm, Decoration, Skill},
	forge::Forge,
};
use std::rc::Rc;
use itertools::Itertools;
use std::fmt;
use crate::forge::types::{ArmorClass};
use std::collections::hash_map::Entry;
use std::ops::Not;
use crate::forge::weapon::Weapon;

#[allow(dead_code)]
enum Sign {
	GE,
	EQ
}

pub struct BestSet {
	pub weapon: Option<Rc<Weapon>>,
	pub set: [Option<Rc<Armor>>; 5],
	charm: Option<Rc<Charm>>,
}

impl BestSet {
	pub fn new() -> Self {
		BestSet {
			weapon: None,
			set: [None, None, None, None, None],
			charm: None
		}
	}

	pub fn add_armor(&mut self, armor: &Rc<Armor>) {
		self.set[armor.class as usize] = Some(Rc::clone(armor));
	}

	pub fn get_skills(&self) -> HashMap<Rc<Skill>, u8> {
		let mut skills_sum: HashMap<Rc<Skill>, u8> = Default::default();
		for i in self.set.iter() {
			if let Some(armor) = i {
				for (skill, lev) in armor.skills.iter() {
					match skills_sum.entry(Rc::clone(skill)) {
						Entry::Occupied(mut o) => o.insert(o.get() + lev),
						Entry::Vacant(v) => *v.insert(*lev)
					};
				}
			}
		}
		skills_sum.shrink_to_fit();
		skills_sum
	}
}

impl fmt::Display for BestSet {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut str = String::new();
		str = format!("Weapon not implemented\n");

		for i in ArmorClass::iterator() {
			str = format!("{} {}:", str, ArmorClass::to_string(i));
			match self.set.get(*i as usize) {
				Some(Some(armor)) => str = format!("{} <{}>\n", str, armor),
				Some(None) => str = format!("{} None\n", str),
				None => panic!("ERROR: Result print out of bounds"),
			}
		}

		if let Some(charm) = &self.charm {
			str = format!("{} Charm: {}", str, charm);
		}

		write!(f, "{}", str)
	}
}


pub struct Searcher {
	forge: Rc<Forge>,
	skills_req: RefCell<HashMap<Rc<Skill>, u8>>,
	armours: RefCell<[Vec<(Rc<Armor>, u8)>; 5]>,
	charms: RefCell<HashMap<Rc<Charm>, u8>>,
	decorations: RefCell<HashMap<Rc<Decoration>, u8>>,
}

impl fmt::Display for Searcher {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut str;
		str = format!("###    ARMORS   ###\n");
		for list in self.armours.borrow().iter() {
			match list.get(0) {
				Some((a, _)) => str = format!("{}##    {}    ##\n", str, ArmorClass::to_string(&a.class)),
				None => ()
			};
			for (armor, rank) in list.iter() {
				str = format!("{} {} : {}\n", str, rank, armor);
			}
		}
		str = format!("{}###    CHARMS   ###\n", str);
		for (charm, rank) in self.charms.borrow().iter() {
			str = format!("{} {} : {}\n", str, rank, charm);
		}
		str = format!("{}###    DECORATIONS   ###\n", str);
		for (deco, rank) in self.decorations.borrow().iter() {
			str = format!("{} {} : {}\n", str, rank, deco);
		}
		write!(f, "{}##################\n", str)
	}
}

impl Searcher {
	pub fn new(forge: Rc<Forge>) -> Self {
		Searcher {
			skills_req: Default::default(),
			armours: Default::default(),
			charms: Default::default(),
			decorations: Default::default(),
			forge,
		}
	}

	pub fn add_skill_requirement(&self, skill: Rc<Skill>, lev: u8) {  // Add Sign >= or =
		if lev == 0 {
			self.skills_req.borrow_mut().remove(&skill);
		} else {
			self.skills_req.borrow_mut().insert(skill.clone(), lev);
		}
	}

	pub fn show_requirements(&self) {
		println!("Requirements: ");
		for (skill, lev) in self.skills_req.borrow().iter() {
			println!("Skill: {} lev: {}", skill.name, lev);
		}
	}

	fn check_req(&self, res: &BestSet) -> bool {
		let mut satisfied = true;
		let set_skills = res.get_skills();
		for (skill, req_lev) in self.skills_req.borrow().iter() {
			match set_skills.get(skill) {
				Some(level) => if req_lev > level { satisfied = false },
				None => satisfied = false,
			}
		}
		satisfied
	}

	fn filter(&self) {
		let mut order: [Vec<(Rc<Armor>, u8)>; 5] = Default::default();
		let filtered = self.forge.get_armors_filtered(&self.skills_req);
		for (armor, rank) in filtered.iter().sorted_by(|(_, a), (_, b)| { b.cmp(&a) }) {
			order[armor.class as usize].push((Rc::clone(armor), *rank));
		}
		self.armours.replace(order);
		self.charms.replace(self.forge.get_charms_filtered(&self.skills_req));
		self.decorations.replace(self.forge.get_decorations_filtered(&self.skills_req));
		println!("ARMORS: {} CHARMS: {}, DECORATIONS: {}", filtered.len(), self.charms.borrow().len(), self.decorations.borrow().len());
	}

	fn stupid_search(&self) -> BestSet {
		let mut res = BestSet::new();
		for i in self.armours.borrow().iter() {
			if self.check_req(&res).not() {
				match i.get(0) {
					Some((armor, _)) => res.add_armor(armor),
					None => (),
				};
			}
		}
		res
	}

	//pub fn search(&self) -> Result {}

	pub fn calculate(&self) -> BestSet {
		self.filter();
		self.stupid_search()
	}
}

#[cfg(test)]
mod tests {
	#![allow(dead_code)]
	use std::rc::Rc;
	use crate::forge;

	#[test]
	fn filtering() {
		let forge = Rc::new(forge::forge::Forge::new());
		forge.load_all("it");
		let searcher = forge::searcher::Searcher::new(Rc::clone(&forge));

		searcher.add_skill_requirement(forge.get_skill("Occhio critico").unwrap(), 3);
		searcher.add_skill_requirement(forge.get_skill("Angelo custode").unwrap(), 1);
		//searcher.add_skill_requirement(forge.get_skill("Critico elementale").unwrap(), 1);

		let res = searcher.calculate();
		//println!("{}", searcher);
		println!("Result:\n{}", res);
		assert_eq!("a", "a");
	}
}