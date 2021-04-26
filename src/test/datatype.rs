use std::{
	borrow::Borrow,
	mem,
	ops::Not,
	rc::Rc,
	sync::{Arc, Mutex, Once},
	thread,
	thread::sleep,
	time::Duration,
	collections::HashMap,
};
use crate::datatypes::{
	forge::Forge,
	equipment::Equipment,
	ID, Level, MAX_SLOTS,
	types::{ArmorClass, Gender, ArmorRank, Item},
	skill::{Skill, SkillLevel, SkillsLevel},
	decoration::AttachedDecorations,
};
use crate::test::Shared;

#[test]
fn attached_decorations() {
	let shared = Shared::get();
	let armors = shared.forge.armors.borrow();
	let armor = armors.get(&1545).unwrap();  // Slots 4-4-2 Skill id 16 level 3
	let mut armdec = AttachedDecorations::new(Arc::clone(armor));
	let decos = shared.forge.decorations.borrow();

	let deco1 = decos.get(&150).unwrap();  // Size 4 Skill id 16  level 3
	let deco2 = decos.get(&149).unwrap();  // Skill id 73 size <3>
	let deco3 = decos.get(&143).unwrap();  // Skill id 86 size <4>
	let deco4 = decos.get(&53).unwrap();   // Skill id 47 size <2>

	assert_eq!(armdec.try_add_deco(Arc::clone(deco1)).is_ok(), true);
	assert_eq!(armdec.try_add_deco(Arc::clone(deco2)).is_ok(), true);
	assert_eq!(armdec.try_add_deco(Arc::clone(deco3)).is_ok(), false);
	assert_eq!(armdec.try_add_deco(Arc::clone(deco4)).is_ok(), true);

	let skills = shared.forge.skills.borrow();
	let mut skill_list = SkillsLevel::new();
	skill_list.update_or_append(SkillLevel::new(Arc::clone(skills.get(&16).unwrap()), 6));
	skill_list.update_or_append(SkillLevel::new(Arc::clone(skills.get(&73).unwrap()), 3));
	skill_list.update_or_append(SkillLevel::new(Arc::clone(skills.get(&47).unwrap()), 1));

	let mut hash: HashMap<ID, Level> = Default::default();
	armdec.get_skills_chained(&mut hash);
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
