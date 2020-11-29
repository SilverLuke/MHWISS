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
use crate::forge::types::{ArmorClass, ID, Level};
use std::collections::hash_map::Entry;
use std::ops::Not;
use crate::forge::weapon::Weapon;
use std::sync::Arc;

#[allow(dead_code)]
enum Sign {
	GE,
	EQ
}

pub struct ArmorDeco {
	armor: Rc<Armor>,
	deco: [Option<Rc<Decoration>>; 3],
}

impl ArmorDeco {
	pub fn new(armor: Rc<Armor>) -> Self {
		ArmorDeco {
			armor: armor,
			deco: [None, None, None]
		}
	}

	pub fn get_armor(&self) -> &Rc<Armor> {
		&self.armor
	}

	fn is_empty(&self, i: usize) -> bool {
		self.deco[i].is_none()
	}

	fn try_add_deco(&mut self, deco: &Rc<Decoration>) -> Result<(), &str> {
		for (i, size) in self.armor.slots.iter().enumerate().rev() {
			if *size >= deco.size {
				if self.is_empty(i) {
					self.deco[i] = Some(Rc::clone(deco));
					return Ok(());
				}
			}
		}
		Err("No space left")
	}

	/*
		fn get_skill(&self) -> HashMap<ID, Level> {
			let mut skills_sum: HashMap<ID, Level> = Default::default();
			for (skill, lev) in self.armor.skills.iter() {
				skills_sum.insert(skill.id, *lev);
			}
			for deco in self.deco.iter() {
				if let Some(deco) = deco {
					for skill in deco.skills.iter() {
						match skills_sum.entry(skill.id) {
							Entry::Occupied(mut o) => o.insert(o.get() + lev),
							Entry::Vacant(v) => *v.insert(*lev)
						};
					}
				}
				break;
			}
			skills_sum
		}
	*/
	fn get_skills(&self, skills_sum: &mut HashMap<ID, Level>) {
		for (skill, lev) in self.armor.skills.iter() {
			match skills_sum.entry(skill.id) {
				Entry::Occupied(mut o) => o.insert(o.get() + lev),
				Entry::Vacant(v) => *v.insert(*lev)
			};
		}
		for deco in self.deco.iter() {
			if let Some(deco) = deco {
				for (skill, lev) in deco.skills.iter() {
					match skills_sum.entry(skill.id) {
						Entry::Occupied(mut o) => o.insert(o.get() + lev),
						Entry::Vacant(v) => *v.insert(*lev)
					};
				}
			}
		}
	}
}

pub struct BestSet {
	pub weapon: Option<Rc<Weapon>>,
	pub set: [Option<ArmorDeco>; 5],
	pub charm: Option<Rc<Charm>>,
}

impl BestSet {
	pub fn new() -> Self {
		BestSet {
			weapon: None,
			set: <[_; 5]>::default(),
			charm: None
		}
	}

	pub fn add_armor(&mut self, armor: &Rc<Armor>) {
		self.set[armor.class as usize] = Some(ArmorDeco::new(Rc::clone(armor)));
	}

	fn get_weapon_skills(&self, skills_sum: &mut HashMap<ID, Level>) {
		if let Some(w) = &self.weapon {
			if let Some(skill) = &w.skill {
				*skills_sum.entry(skill.id).or_insert(1) += 1;
			}
		}
	}

	fn get_armors_skills(&self, skills_sum: &mut HashMap<ID, Level>) {
		for i in self.set.iter() {
			if let Some(armor) = i {
				armor.get_skills(skills_sum);
			}
		}
	}

	fn get_charm_skills(&self, skills_sum: &mut HashMap<ID, Level>) {
		if let Some(charm) = &self.charm {
			for (skill, lev) in charm.skills.iter() {
				*skills_sum.entry(skill.id).or_insert(*lev) += *lev;
			}
		}
	}

	pub fn get_skills(&self) -> HashMap<ID, Level> {
		let mut skills_sum: HashMap<ID, Level> = Default::default();

		self.get_weapon_skills(&mut skills_sum);
		self.get_armors_skills(&mut skills_sum);
		self.get_charm_skills(&mut skills_sum);
		skills_sum.shrink_to_fit();
		skills_sum
	}
}

impl fmt::Display for BestSet {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut str;
		str = match &self.weapon {
			Some(w) => format!("Weapon: {}\n", w),
			None => format!("Weapon: None\n")
		};

		for i in ArmorClass::iterator() {
			str = format!("{} {}:", str, ArmorClass::to_string(i));
			match self.set.get(*i as usize) {
				Some(Some(armor)) => str = format!("{} <{}>\n", str, armor.armor),
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
	forge: Arc<Forge>,
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
	pub fn new(forge: Arc<Forge>) -> Self {
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
			match set_skills.get(&skill.id) {
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
	use std::rc::Rc;
	use crate::forge;
	use crate::forge::searcher::{ArmorDeco, Searcher};
	use crate::forge::forge::Forge;
	use std::borrow::Borrow;
	use std::sync::{Arc, Mutex, Once};
	use std::time::Duration;
	use std::{mem, thread};
	use std::thread::sleep;
	use crate::forge::types::SkillsLev;

	#[derive(Clone)]
	struct Shared {
		forge: Arc<Forge>,
	}

	impl Shared {
		fn new() -> Self {
			let forge = Forge::new();
			forge.load_all("it");
			Shared {
				forge: Arc::new(forge),
			}
		}

		pub fn get() -> Self {
			static mut SINGLETON: *const Shared = 0 as *const Shared;
			static ONCE: Once = Once::new();
			unsafe {
				ONCE.call_once(|| {
					let singleton = Shared::new();
					SINGLETON = mem::transmute(Box::new(singleton));
				});
				(*SINGLETON).clone()
			}
		}
	}

	#[test]
	fn filtering() {
		let shared = Shared::get();
		let forge = shared.forge;
		let searcher = Searcher::new(forge.clone());

		searcher.add_skill_requirement(forge.get_skill("Occhio critico").unwrap(), 3);
		searcher.add_skill_requirement(forge.get_skill("Angelo custode").unwrap(), 1);
		//searcher.add_skill_requirement(forge.get_skill("Critico elementale").unwrap(), 1);

		let res = searcher.calculate();
		println!("Result:\n{}", res);
		assert_eq!("a", "a");

	}

	#[test]
	fn armordeco() {
		let shared = Shared::get();
		let armors = shared.forge.armors.borrow();
		let armor = armors.get(&1545).unwrap();  // Slots 4-4-2 Skill id 16 <3>
		let mut armdec = ArmorDeco::new(Rc::clone(armor));
		let decos = shared.forge.decorations.borrow();

		let deco1 = decos.get(&150).unwrap();  // Skill id 16 <3>
		let deco2 = decos.get(&149).unwrap();  // Skill id 73 <3>
		let deco3 = decos.get(&143).unwrap();  // Skill id 86 <4>
		let deco4 = decos.get(&53).unwrap();   // Skill id 47 <2>

		assert_eq!(armdec.try_add_deco(deco1).is_ok(), true);
		assert_eq!(armdec.try_add_deco(deco2).is_ok(), true);
		assert_eq!(armdec.try_add_deco(deco3).is_ok(), false);
		assert_eq!(armdec.try_add_deco(deco4).is_ok(), true);

		let skills = shared.forge.skills.borrow();
		let skill_list:SkillsLev = vec![
			(skills.get(&16).unwrap().clone(), 6),
			(skills.get(&73).unwrap().clone(), 3),
			(skills.get(&47).unwrap().clone(), 1),
		];

		let mut hash = Default::default();
		armdec.get_skills(&mut hash);
		assert_eq!(hash.len(), skill_list.len());
		for (skill, lev) in skill_list.iter() {
			if let Some(ad_lev) = hash.get(&skill.id) {
				if ad_lev != lev {
					assert!(false);
				}
			} else {
				assert!(false);
			}
		}
	}
}