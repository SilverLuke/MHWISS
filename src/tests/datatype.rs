use std::{
	borrow::Borrow,
	sync::Arc,
};
use crate::data::{
	mutable::attached_decorations::AttachedDecorations,
	db_types::{
		HasSkills,
		skill::{SkillLevel, SkillsLevel},
	},
	db::{get_armor_by_id, get_decorations_by_id, get_skill_by_id},
};
use crate::tests::Shared;

#[test]
fn attached_decorations() {
	let shared = Shared::get();
	let armors = &shared.storage.armors;
	let armor = get_armor_by_id(armors, 1545).unwrap();  // Slots 4-4-2 Skill id 16 level 3
	let mut armdec = AttachedDecorations::new(Arc::clone(armor));
	let decorations = shared.storage.decorations.borrow();

	let deco1 = get_decorations_by_id(decorations, 150).unwrap();  // Size 4 Skill id 16  level 3
	let deco2 = get_decorations_by_id(decorations, 149).unwrap();  // Skill id 73 size <3>
	let deco3 = get_decorations_by_id(decorations, 143).unwrap();  // Skill id 86 size <4>
	let deco4 = get_decorations_by_id(decorations, 53).unwrap();   // Skill id 47 size <2>

	assert_eq!(armdec.try_add_deco(Arc::clone(deco1)).is_ok(), true);
	assert_eq!(armdec.try_add_deco(Arc::clone(deco2)).is_ok(), true);
	assert_eq!(armdec.try_add_deco(Arc::clone(deco3)).is_ok(), false);
	assert_eq!(armdec.try_add_deco(Arc::clone(deco4)).is_ok(), true);

	let skills = shared.storage.skills.borrow();
	let mut skill_list = SkillsLevel::new();
	skill_list.insert(SkillLevel::new(Arc::clone(get_skill_by_id(skills,16).unwrap()), 6));
	skill_list.insert(SkillLevel::new(Arc::clone(get_skill_by_id(skills,73).unwrap()), 3));
	skill_list.insert(SkillLevel::new(Arc::clone(get_skill_by_id(skills,47).unwrap()), 1));

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
