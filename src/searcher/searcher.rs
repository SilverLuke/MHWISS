use std::{
	fmt,
	rc::Rc,
	ops::Not,
	sync::Arc,
	cell::RefCell,
	collections::{
		hash_map::Entry,
		HashMap,
	},
};
use itertools::Itertools;
use crate::forge::{
	armor::Armor,
	skill::{Charm, Decoration, Skill},
	forge::Forge,
	weapon::Weapon,
	types::{ArmorClass, ID, Level},
};
use crate::searcher::{
	container::{HasDecorations, DecorationContainer, HasSkills},
	bestset::BestSet,
};
use std::borrow::Borrow;
use crate::forge::types::Decorations;
use std::cmp::Ordering;

pub struct Searcher {
	forge: Arc<Forge>,
	input_skills_req: RefCell<HashMap<ID, u8>>,
	skills_req: RefCell<HashMap<ID, u8>>,
	armors: RefCell<Vec<DecorationContainer<Armor>>>,
	charms: RefCell<Vec<(Rc<Charm>, u8)>>,
	decorations: RefCell<Vec<(Rc<Decoration>, u8)>>,
}

impl fmt::Display for Searcher {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut str;
		str = format!("###    ARMORS   ###\n");
		for elem in self.armors.borrow().iter() {
			str = format!("{} [{}] {}\n", str, elem.value, elem.container);
		}
		str = format!("{}###    CHARMS   ###\n", str);
		for (charm, rank) in self.charms.borrow().iter() {
			str = format!("{} [{}] {}\n", str, rank, charm);
		}
		str = format!("{}###    DECORATIONS   ###\n", str);
		for (deco, rank) in self.decorations.borrow().iter() {
			str = format!("{} [{}] {}\n", str, rank, deco);
		}
		write!(f, "{}##################\n", str)
	}
}

impl Searcher {
	pub fn new(forge: Arc<Forge>) -> Self {
		Searcher {
			input_skills_req: Default::default(),
			skills_req: Default::default(),
			armors: Default::default(),
			charms: Default::default(),
			decorations: Default::default(),
			forge,
		}
	}

	pub fn show_requirements(&self) {
		println!("Requirements: ");
		for (id, lev) in self.input_skills_req.borrow().iter() {
			println!("Skill: {} lev: {}", self.forge.skills.get(id).unwrap().name, lev);
		}
	}

	pub fn add_skill_requirement(&self, skill_id: ID, lev: u8) {  // Add Sign >= or =
		if lev == 0 {
			self.input_skills_req.borrow_mut().remove(&skill_id);
		} else {
			self.input_skills_req.borrow_mut().insert(skill_id, lev);
		}
	}

	pub(crate) fn check_requirements(&self, res: &BestSet) -> bool {
		let mut satisfied = true;
		let set_skills = res.get_skills();
		for (skill_id, req_lev) in self.input_skills_req.borrow().iter() {
			match set_skills.get(skill_id) {
				Some(level) => {
					if req_lev > level {
						satisfied = false
					}
				},
				None => satisfied = false,
			}
		}
		satisfied
	}

	fn remove_requirements(&self, skills: HashMap<ID, Level>) {
		for (id, val) in skills {
			match self.skills_req.borrow_mut().entry(id) {
				Entry::Occupied(mut o) => {
					let remaining = o.get() - val;
					if remaining <= 0 {
						o.remove();
					} else {
						o.insert(remaining);
					}
				},
				Entry::Vacant(v) => println!("Skill {} not in requirements", self.forge.skills.get(&id).unwrap().name)
			};
		}
	}

	fn get_armor_value(&self, armor: &Rc<Armor>) -> Level {
		42
	}

	fn get_armors_filtered(&self) -> Vec<(Rc<Armor>, Level)> {
		let list = self.forge.get_armors_filtered(&self.skills_req);
		let mut ret = vec![];
		for e in list {
			let value = self.get_armor_value(&e);
			ret.push((e, value));
		}
		ret
	}

	fn print_requirements(&self) {
		println!("Requirements:");
		for (key, val) in self.skills_req.borrow().iter() {
			println!("\t{0: <40} {1: <2} ", self.forge.skills.get(key).unwrap(), val);
		}
	}

	fn print_filter(&self) {
		println!("Armors:");
		for i in self.armors.borrow().iter() {
			if i.value > 0 {
				println!("\t{}", i.to_string());
			}
		}
		println!("Charms:",);
		for (c, val) in self.charms.borrow().iter() {
			if *val > 0 {
				println!("\t{0:<50} | {1:<2}", c.to_string(), val);
			}
		}
		println!("Decorations:",);
		for (d, val) in self.decorations.borrow().iter() {
			if *val > 0 {
				println!("\t{0:<50} | {1:<2}", d.to_string(), val);
			}
		}
	}

	fn filter(&self) {
		self.charms.replace(self.forge.get_charms_filtered(&self.skills_req));
		self.decorations.replace(self.forge.get_decorations_filtered(&self.skills_req));

		let mut vec = vec![];
		for (_, piece) in self.forge.armors.iter() {
			let mut tmp = DecorationContainer::new(Rc::clone(piece));
			tmp.value(&*self.skills_req.borrow());
			vec.push(tmp);
		}
		vec.sort_by(|a, b| {
			let value = b.value.cmp(&a.value);
			if  value == Ordering::Equal {
				b.container.defence[2].cmp(&a.container.defence[2])
			} else {
				value
			}
		});

		self.armors.replace(vec);
	}

	fn stupid_search(&self) -> BestSet {
		let mut result = BestSet::new();
		while self.check_requirements(&result).not() && result.is_full().not() {
			self.filter();
			self.print_requirements();
			self.print_filter();
			let mut i = 0;
			let mut done = true;
			while done {
				let tmp = self.armors.borrow();
				let piece = tmp.get(i).unwrap();
				if result.try_add_armor(&piece).is_ok() {
					self.remove_requirements(piece.get_skills());
					done = false;
				} else {
					i += 1;
				}
			}
		}
		result
	}

	fn init(&self) {
		let tmp = self.input_skills_req.borrow().clone();
		self.skills_req.replace(tmp);
	}

	pub fn calculate(&self) -> BestSet {
		self.init();
		self.stupid_search()
	}
}

#[cfg(test)]
#[path = "./tests.rs"]
mod tests;