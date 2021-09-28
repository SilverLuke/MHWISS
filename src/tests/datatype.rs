use std::{
	borrow::Borrow,
	sync::Arc,
};
use crate::data::{
	mutable::attached_decorations::AttachedDecorations,
	db_types::{
		skill::{SkillLevel, SkillsLevel},
	}
};
use crate::data::db_types::HasSkills;
use crate::tests::Shared;

#[test]
fn attached_decorations() {
	let shared = Shared::get();
	let armors = shared.storage.armors.borrow();
	let armor = armors.get(&1545).unwrap();  // Slots 4-4-2 Skill id 16 level 3
	let mut armdec = AttachedDecorations::new(Arc::clone(armor));
	let decos = shared.storage.decorations.borrow();

	let deco1 = decos.get(&150).unwrap();  // Size 4 Skill id 16  level 3
	let deco2 = decos.get(&149).unwrap();  // Skill id 73 size <3>
	let deco3 = decos.get(&143).unwrap();  // Skill id 86 size <4>
	let deco4 = decos.get(&53).unwrap();   // Skill id 47 size <2>

	assert_eq!(armdec.try_add_deco(Arc::clone(deco1)).is_ok(), true);
	assert_eq!(armdec.try_add_deco(Arc::clone(deco2)).is_ok(), true);
	assert_eq!(armdec.try_add_deco(Arc::clone(deco3)).is_ok(), false);
	assert_eq!(armdec.try_add_deco(Arc::clone(deco4)).is_ok(), true);

	let skills = shared.storage.skills.borrow();
	let mut skill_list = SkillsLevel::new();
	skill_list.insert(SkillLevel::new(Arc::clone(skills.get(&16).unwrap()), 6));
	skill_list.insert(SkillLevel::new(Arc::clone(skills.get(&73).unwrap()), 3));
	skill_list.insert(SkillLevel::new(Arc::clone(skills.get(&47).unwrap()), 1));

	let skills_level = armdec.get_skills();
	assert_eq!(skills_level.len(), skill_list.len());
	for skill in skill_list.iter() {
		if let Some(ad_lev) = skills_level.get_level(skill.get_skill()) {
			if ad_lev != skill.get_level() {
				assert!(false);
			}
		} else {
			assert!(false);
		}
	}
	assert!(true);
}
