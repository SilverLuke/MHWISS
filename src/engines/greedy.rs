use std::{
	fmt,
	cell::RefCell,
	ops::Not,
	rc::Rc,
	sync::Arc,
	cmp::Ordering,
	collections::{HashMap, hash_map::Entry}
};
use crate::datatypes::{
	Level, ID,
	decoration::{AttachedDecorations, Decoration},
	equipment::Equipment,
	armor::Armor,
	forge::Forge,
	charm::Charm,
};
use crate::engines::{EnginesManager, Engine};
use crate::datatypes::skill::SkillsLevel;
/*
struct MagicValue<T> {
	pub item: Arc<T>,
	pub value: i16,
}

trait Item {
	fn get_skills(&self) -> SkillsLevel;
	fn get_slots(&self) -> [u8; MAX_SLOTS];
}

impl MagicValue<T> where T: Item {
	fn new(item: T) -> Self {
		let value = item.get_skills();
		MagicValue {
			item: Arc::new(item),
			value,
		}
	}

}
*/
pub(crate) struct Greedy {
	// Engine Related
	forge: Arc<Forge>,
	constrains: HashMap<ID, Level>,
	// Greedy related
	current_constrains: HashMap<ID, Level>,
	armors: RefCell<Vec<Arc<Armor>>>,
	charms: RefCell<Vec<Arc<Charm>>>,
	decorations: RefCell<Vec<Arc<Decoration>>>,
}

impl Greedy {
	fn remove_constrains(&self, skills: HashMap<ID, Level>) {
		/*
		for (id, val) in skills {
			match self.constrains.borrow_mut().entry(id) {
				Entry::Occupied(mut o) => {
					let remaining = o.get() - val;
					if remaining <= 0 {
						o.remove();
					} else {
						o.insert(remaining);
					}
				}
				Entry::Vacant(v) => println!("Skill {} not in requirements", self.engines.forge.skills.get(&id).unwrap().name)
			};
		}
		*/
		todo!()
	}
/*
	fn print_filter(&self) {
		println!("Armors:");
		for i in self.armors.borrow().iter() {
			if i.value > 0 {
				println!("\t{}", i.to_string());
			}
		}
		println!("Charms:");
		for (c, val) in self.charms.borrow().iter() {
			if *val > 0 {
				println!("\t{0:<50} | {1:<2}", c.to_string(), val);
			}
		}
		println!("Decorations:");
		for (d, val) in self.decorations.borrow().iter() {
			if *val > 0 {
				println!("\t{0:<50} | {1:<2}", d.to_string(), val);
			}
		}
	}
	*/

	fn filter(&self) {
		return;
	}

	fn init(&self) {
		let mut tmp = self.forge.get_charms_filtered(&self.current_constrains);
		//tmp.sort_by(|a, b| { b.cmp(&a) });
		self.charms.replace(tmp);
		let mut tmp = self.forge.get_decorations_filtered(&self.current_constrains);
		//tmp.sort_by(|a, b| { b.1.cmp(&a.1) });
		self.decorations.replace(tmp);

		let mut vec = vec![];
		for (_, piece) in self.forge.armors.iter() {
			// let mut tmp = AttachedDecorations::new(Arc::clone(piece));
			// tmp.value(&*self.skills_req.borrow());
			vec.push(Arc::clone(piece));
		}
		/*
		vec.sort_by(|a, b| {
			let value = b.value.cmp(&a.value);
			if value == Ordering::Equal {
				b.item.defence[2].cmp(&a.item.defence[2])
			} else {
				value
			}
		});
		*/
		self.armors.replace(vec);

	}

	pub fn check_constrains(&self, res: &Equipment) -> bool {
		let mut satisfied = true;
		let set_skills = res.get_skills();
		for (skill_id, req_lev) in self.constrains.iter() {
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
}

impl fmt::Debug for Greedy {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut str;
		str = format!("###    ARMORS   ###\n");
		/*
		for armors_list in self.armors.iter() {
			for elem in armors_list.iter() {
				str = format!("{} [{}] {}\n", str, elem.value, elem.item);
			}
		}
		*/
		str = format!("{}###    CHARMS   ###\n", str);
		/*
		for (charm, rank) in self.charms.iter() {
			str = format!("{} [{}] {}\n", str, rank, charm);
		}
		*/
		str = format!("{}###    DECORATIONS   ###\n", str);
		/*
		for (deco, rank) in self.decorations.iter() {
			str = format!("{} [{}] {}\n", str, rank, deco);
		}
		*/
		write!(f, "{}##################\n", str)
	}
}

impl Engine for Greedy {
	fn new(forge: Arc<Forge>, constrains: HashMap<ID, Level>) -> Self {
		let copy = constrains.clone();
		Greedy {
			forge,
			constrains,
			current_constrains: copy,
			armors: Default::default(),
			charms: Default::default(),
			decorations: Default::default()
		}
	}

	fn run(&self) -> Equipment {
		self.init();
		let mut result = Equipment::new();
		let mut impossible = false;
		while self.check_constrains(&result).not() && result.is_full().not() && impossible.not() {
			self.filter();
			let mut i = 0;
			let mut done = true;
			while done {  // Loop until a armor is suited for placement
				let tmp = self.armors.borrow();
				match tmp.get(i) {
					Some(piece) =>
						if result.try_add_armor(&AttachedDecorations::new(Arc::clone(piece))).is_ok() {
							//self.remove_requirements(piece.get_skills());
							done = false;  // Go for the next piece
						} else {
							i += 1;
						},
					None => {
						done = false;  // No more armor to be tested
						impossible = true;  // So no configuration exist (Maybe???)
					}
				};
			}
		}
		result
	}
}