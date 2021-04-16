use std::{
	borrow::Borrow,
	mem,
	ops::Not,
	rc::Rc,
	sync::{Arc, Mutex, Once},
	thread,
	thread::sleep,
	time::Duration
};

use crate::datatypes::{
	forge::Forge,
	equipment::Equipment,
	ID, Level, MAX_SLOTS,
	types::{ArmorClass, Gender, Rank},
	skill::{Skill, SetSkill, HasSkills, SkillLevel, SkillsLevel},
	decoration::{HasDecorations, AttachedDecorations},
};
use crate::engines::{
	EnginesManager,
	greedy::Greedy,
	genetic::Genetic,
	Engine
};

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
fn greedy() {
	println!("TEST: greedy");
	let shared = Shared::get();
	let forge = shared.forge;
	let searcher = EnginesManager::new(forge.clone());

	searcher.add_constraint(forge.get_skill_from_name("Battitore").unwrap().id, 5);
	//engines.add_skill_requirement(datatypes.get_skill("Angelo custode").unwrap(), 1);
	//engines.add_skill_requirement(datatypes.get_skill("Critico elementale").unwrap(), 1);

	let engine = Greedy::new(Arc::clone(&forge), searcher.skills_constraints.borrow().clone());
	let res = engine.run();
	println!("Result:\n{}", res);
	assert!(true);
}


#[test]
fn genetic() {
	println!("TEST: genetic");
	let shared = Shared::get();
	let forge = shared.forge;
	let searcher = EnginesManager::new(forge.clone());

	searcher.add_constraint(forge.get_skill_from_name("Battitore").unwrap().id, 5);
	//engines.add_skill_requirement(datatypes.get_skill("Angelo custode").unwrap(), 1);
	//engines.add_skill_requirement(datatypes.get_skill("Critico elementale").unwrap(), 1);

	let engine = Genetic::new(Arc::clone(&forge), searcher.skills_constraints.borrow().clone());
	let res = engine.run();
	println!("Result:\n{}", res);
	assert!(true);
}

#[test]
fn attached_decorations() {
	let shared = Shared::get();
	let armors = shared.forge.armors.borrow();
	let armor = armors.get(&1545).unwrap();  // Slots 4-4-2 Skill id 16 <3>
	let mut armdec = AttachedDecorations::new(Arc::clone(armor));
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
	let mut skill_list = SkillsLevel::new();
	skill_list.update_or_append(SkillLevel::new(Arc::clone(skills.get(&16).unwrap()), 6));
	skill_list.update_or_append(SkillLevel::new(Arc::clone(skills.get(&73).unwrap()), 3));
	skill_list.update_or_append(SkillLevel::new(Arc::clone(skills.get(&47).unwrap()), 1));

	let hash = armdec.get_skills();
	assert_eq!(hash.len(), skill_list.len());
	for skill in skill_list.get_skills() {
		if let Some(ad_lev) = hash.get(&skill.get_id()) {
			if *ad_lev != skill.level {
				assert!(false);
			}
		} else {
			assert!(false);
		}
	}
	assert!(true);
}

