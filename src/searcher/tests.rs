use std::{
	rc::Rc,
	borrow::Borrow,
	sync::{Arc, Mutex, Once},
	time::Duration,
	mem,
	thread,
	thread::sleep,
	ops::Not
};

use crate::searcher::{
	searcher::Searcher,
	bestset::BestSet
};
use crate::datatypes::{
	forge::Forge,
	SkillsLev,
};

use super::*;

#[derive(Clone)]
struct Shared {
	forge: Arc<Forge>,
}

impl Shared {
	fn new() -> Self {
		let mut forge = Forge::new();
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
fn check_requirement() {
	println!("TEST\ncheck_requirement START");
	let shared = Shared::get();
	let forge = shared.forge;
	let searcher = Searcher::new(forge.clone());
	let mut result = BestSet::new();

	searcher.add_skill_requirement(forge.get_skill("Battitore").unwrap().id, 5);
	searcher.init();
	assert_eq!(searcher.check_requirements(&result), false);
	let armor = forge.armors.get(&1550).unwrap();
	let dc = AttachedDecorations::new(armor.clone());
	assert_eq!(result.try_add_armor(&dc).is_ok(), true);
	assert_eq!(result.try_add_armor(&dc).is_ok(), false);
	assert_eq!(searcher.check_requirements(&result), false);
	let armor = forge.armors.get(&1548).unwrap();
	let dc = AttachedDecorations::new(armor.clone());
	assert_eq!(result.try_add_armor(&dc).is_ok(), true);
	assert_eq!(result.try_add_armor(&dc).is_ok(), false);
	assert_eq!(searcher.check_requirements(&result), true);
	println!("check_requirement END");
}

#[test]
fn calculate() {
	println!("TEST\ncalculate START");
	let shared = Shared::get();
	let forge = shared.forge;
	let searcher = Searcher::new(forge.clone());

	searcher.add_skill_requirement(forge.get_skill("Battitore").unwrap().id, 5);
	//searcher.add_skill_requirement(datatypes.get_skill("Angelo custode").unwrap(), 1);
	//searcher.add_skill_requirement(datatypes.get_skill("Critico elementale").unwrap(), 1);

	let res = searcher.calculate();
	println!("Result:\n{}", res);
	assert_eq!(true, true);
	println!("calculate END");
}

#[test]
fn armordeco() {
	println!("TEST");
	println!("Armordeco START");
	let shared = Shared::get();
	let armors = shared.forge.armors.borrow();
	let armor = armors.get(&1545).unwrap();  // Slots 4-4-2 Skill id 16 <3>
	let mut armdec = AttachedDecorations::new(Rc::clone(armor));
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
	let skill_list: SkillsLev = vec![
		(skills.get(&16).unwrap().clone(), 6),
		(skills.get(&73).unwrap().clone(), 3),
		(skills.get(&47).unwrap().clone(), 1),
	];

	let hash = armdec.get_skills();
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

