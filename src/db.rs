use std::{
	str::FromStr,
	borrow::{Borrow, BorrowMut},
	cell::RefCell,
	sync::Arc,
};
use rusqlite::{Connection, params, Row};

use crate::datatypes::{
	*,
	charm::Charm,
	decoration::Decoration,
	skill::{SetSkill, Skill},
	types::{ArmorClass, ArmorRank, ElderSeal, Element, Gender, WeaponClass},
	weapon::Weapon
};
use crate::datatypes::armor::{Armor, ArmorSet};
use crate::datatypes::skill::{SkillLevel, SkillsLevel};
/*
fn make_ascii_titlecase(s: &mut str) -> &str {
	if let Some(r) = s.get_mut(0..1) {
		r.make_ascii_uppercase();
	}
	s
}*/

pub struct DB {
	connection: rusqlite::Connection,
	lang: RefCell<String>,
}

impl DB {
	pub fn new() -> Self {
		let conn = Connection::open_with_flags("MHWorldData/mhw.db", rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();

		DB {
			connection: conn,
			lang: RefCell::new(String::from("en")),
		}
	}

	pub fn load_languages(&self) -> Vec<(String, String)> {
		let mut statement = self.connection.prepare(
			"SELECT id, name FROM language;").unwrap();
		let mut rows = statement.query(params![]).unwrap();
		let mut ret = Vec::new();
		while let Some(row) = rows.next().unwrap() {
			let id: String = row.get(row.column_index("id").unwrap()).unwrap();
			let name: String = row.get(row.column_index("name").unwrap()).unwrap();
			ret.push((id, name));
		}
		ret
	}

	pub fn set_lang(&self, lang: String) {
		self.lang.replace(lang);
	}

	pub fn load_skills(&self, skills: &mut Skills) {
		let mut statement = self.connection.prepare(
			"SELECT s.id, max_level, secret, unlocks_id, name, description
FROM skilltree AS s
JOIN skilltree_text ON skilltree_text.id = s.id
WHERE skilltree_text.lang_id = ?1
ORDER BY unlocks_id;").unwrap();
		let str = &*self.lang.borrow();
		let mut rows = statement.query(params![str]).unwrap();

		while let Some(row) = rows.next().unwrap() {
			let unlock = row.get(row.column_index("unlocks_id").unwrap());
			let unlock = {
				if unlock.is_ok() {
					Some(Arc::clone(skills.get(&unlock.unwrap()).unwrap()))
				} else {
					None
				}
			};
			let id = row.get(row.column_index("id").unwrap()).unwrap();
			let skill = Skill::new(
				id,
				row.get(row.column_index("name").unwrap()).unwrap(),
				row.get(row.column_index("description").unwrap()).unwrap(),
				row.get(row.column_index("max_level").unwrap()).unwrap(),
				row.get(row.column_index("secret").unwrap()).unwrap(),
				unlock,
			);
			skills.insert(id, Arc::new(skill));
		}
	}

	pub fn load_setskills(&self, setskills: &mut SetSkills, skills: &Skills) {
		let mut statement = self.connection.prepare(
			"SELECT setbonus_id, skilltree_id, required, name
		FROM armorset_bonus_skill AS abs
		JOIN armorset_bonus_text AS abt ON abs.setbonus_id = abt.id
		WHERE lang_id = ?1
		ORDER BY setbonus_id;").unwrap();
		let str = &*self.lang.borrow();
		let mut rows = statement.query(params![str]).unwrap();

		fn new_setskill(row: &Row, skills: &Skills) -> SetSkill {
			let id = row.get(row.column_index("setbonus_id").unwrap()).unwrap();
			let skill_id = row.get(row.column_index("skilltree_id").unwrap()).unwrap();
			let req = row.get(row.column_index("required").unwrap()).unwrap();
			let name = row.get(row.column_index("name").unwrap()).unwrap();
			let mut tmp = SetSkill::new(id, name);
			tmp.add_skill(skills.borrow().get(&skill_id).unwrap(), req);
			tmp
		}

		let row = rows.next().unwrap().unwrap();
		let mut id;
		let mut setskill = new_setskill(row, skills);

		while let Some(row) = rows.next().unwrap() {
			id = row.get(row.column_index("setbonus_id").unwrap()).unwrap();
			if setskill.id == id {
				let skill_id = row.get(row.column_index("skilltree_id").unwrap()).unwrap();
				let req = row.get(row.column_index("required").unwrap()).unwrap();
				setskill.add_skill(skills.borrow().get(&skill_id).unwrap(), req);
			} else {
				setskills.insert(setskill.id, Arc::new(setskill));
				setskill = new_setskill(row, skills);
			}
		}
		setskills.borrow_mut().insert(setskill.id, Arc::new(setskill));
	}

	pub fn load_armors(&self, armors: &mut Armors, skills: &Skills, setskills: &SetSkills) {
		let mut statement = self.connection.prepare(
			"SELECT armor.id, name, rank, armor_type,
					armorset_id, armorset_bonus_id,
					male, female,
					slot_1, slot_2, slot_3,
					defense_base, defense_max, defense_augment_max, Fire, water, thunder, ice, dragon,
					skilltree_id, level
				FROM armor
					LEFT JOIN armor_skill ON armor_skill.armor_id == armor.id
					JOIN      armor_text ON armor.id = armor_text.id
				WHERE lang_id =  ?1;").unwrap();
		let str = &*self.lang.borrow();
		let mut rows = statement.query(params![str]).unwrap();

		fn new_armor(row: &Row, skills: &Skills, setskills: &SetSkills) -> Armor {
			let slots = [row.get(row.column_index("slot_1").unwrap()).unwrap(),
				row.get(row.column_index("slot_2").unwrap()).unwrap(),
				row.get(row.column_index("slot_3").unwrap()).unwrap()];
			let defence = [row.get(row.column_index("defense_base").unwrap()).unwrap(),
				row.get(row.column_index("defense_max").unwrap()).unwrap(),
				row.get(row.column_index("defense_augment_max").unwrap()).unwrap()];
			let elements = [row.get(row.column_index("fire").unwrap()).unwrap(),
				row.get(row.column_index("water").unwrap()).unwrap(),
				row.get(row.column_index("thunder").unwrap()).unwrap(),
				row.get(row.column_index("ice").unwrap()).unwrap(),
				row.get(row.column_index("dragon").unwrap()).unwrap(),
			];
			let gender = Gender::new(row.get(row.column_index("male").unwrap()).unwrap(), row.get(row.column_index("female").unwrap()).unwrap());
			let class = ArmorClass::from_str(row.get::<usize, String>(row.column_index("armor_type").unwrap()).unwrap().as_str()).expect("Parse error");
			let mut armor = Armor::new(
				row.get(row.column_index("id").unwrap()).unwrap(),
				row.get(row.column_index("name").unwrap()).unwrap(),
				class,
				gender,
				slots,
				defence,
				elements,
			);
			let skill_id = row.get(row.column_index("skilltree_id").unwrap());
			let skill_lev = row.get(row.column_index("level").unwrap());
			if skill_id.is_ok() {
				armor.add_skill(skills.get(&skill_id.unwrap()).unwrap(), skill_lev.unwrap());
			}
			let setskill_id = row.get(row.column_index("armorset_bonus_id").unwrap());
			if let Ok(id) = setskill_id {
				armor.add_setskill(setskills.borrow().get(&id).unwrap());
			}
			armor
		}

		let row = rows.next().unwrap().unwrap();
		let mut armor = new_armor(row, &skills, setskills);
		let mut id;

		while let Some(row) = rows.next().unwrap() {
			id = row.get(row.column_index("id").unwrap()).unwrap();
			if armor.id == id {
				let skill_id = row.get(row.column_index("skilltree_id").unwrap()).unwrap();
				let skill_lev = row.get(row.column_index("level").unwrap()).unwrap();
				armor.add_skill(skills.borrow().get(&skill_id).unwrap(), skill_lev);
			} else {
				armor.skills.shrink_to_fit();
				armors.borrow_mut().insert(armor.id, Arc::new(armor));
				armor = new_armor(row, &skills, setskills);
			}
		}
		armor.skills.shrink_to_fit();
		armors.borrow_mut().insert(armor.id, Arc::new(armor));
	}

	pub fn load_sets(&self, sets: &mut Sets, armors: &Armors, setskills: &SetSkills) {
		let mut statement = self.connection.prepare(
			"SELECT armorset.id AS armorset_id, armor.id AS armor_id, armorset_text.name, armorset.rank, armor.armorset_bonus_id
FROM armorset
	 JOIN armor ON armorset.id == armor.armorset_id
	 JOIN armorset_text ON armorset_text.id == armorset.id
WHERE armorset_text.lang_id = ?1;").unwrap();
		let str = &*self.lang.borrow();
		let mut rows = statement.query(params![str]).unwrap();

		fn new_set(row: &Row, armors: &Armors, setskills: &SetSkills) -> ArmorSet {
			let id = row.get(row.column_index("armorset_id").unwrap()).unwrap();
			let armor_id = row.get(row.column_index("armor_id").unwrap()).unwrap();
			let name = row.get(row.column_index("name").unwrap()).unwrap();
			let rank: ArmorRank = ArmorRank::from_str(
				row.get::<usize, String>(row.column_index("rank").unwrap())
					.unwrap()
					.as_str()
			).expect("Parse Error");
			let skill = {
				if let Ok(id) = row.get(row.column_index("armorset_bonus_id").unwrap()) {
					Some(Arc::clone(setskills.borrow().get(&id).unwrap()))
				} else { None }
			};
			let mut set = ArmorSet::new(id, name, rank, skill);
			set.add_armor(armors.borrow().get(&armor_id).unwrap());
			set
		}

		let row = rows.next().unwrap().unwrap();
		let mut id = 0;
		let mut set = new_set(row, &armors, &setskills);

		while let Some(row) = rows.next().unwrap() {
			id = row.get(row.column_index("armorset_id").unwrap()).unwrap();
			if set.id == id {
				let armor_id = row.get(row.column_index("armor_id").unwrap()).unwrap();
				set.add_armor(armors.borrow().get(&armor_id).unwrap());
			} else {
				sets.borrow_mut().insert(set.id, Arc::new(set));
				set = new_set(row, &armors, &setskills);
			}
		}
		sets.borrow_mut().insert(id, Arc::new(set));
	}

	pub fn load_decorations(&self, decorations: &mut Decorations, skills: &Skills) {
		let mut statement = self.connection.prepare(
			"SELECT decoration.id, name, slot, skilltree_id, skilltree_level, skilltree2_id, skilltree2_level
				 FROM decoration
				 JOIN decoration_text ON decoration_text.id == decoration.id
				 WHERE lang_id == ?1").unwrap();
		let str = &*self.lang.borrow();
		let mut rows = statement.query(params![str]).unwrap();

		while let Some(row) = rows.next().unwrap() {
			let mut deco_skills = SkillsLevel::new();

			let skill_id = &row.get(row.column_index("skilltree_id").unwrap()).unwrap();
			let skill = Arc::clone(skills.borrow().get(skill_id).unwrap());
			let level = row.get(row.column_index("skilltree_level").unwrap()).unwrap();
			deco_skills.update_or_append(SkillLevel::new(skill, level));

			if let Ok(tmp) = row.get(row.column_index("skilltree2_id").unwrap()) {
				let skill = Arc::clone(skills.borrow().get(&tmp).unwrap());
				let level = row.get(row.column_index("skilltree2_level").unwrap()).unwrap();
				deco_skills.update_or_append(SkillLevel::new(skill, level));
			}

			let id = row.get(row.column_index("id").unwrap()).unwrap();
			decorations.borrow_mut().insert(id, Arc::new(Decoration::new(
				id,
				row.get(row.column_index("name").unwrap()).unwrap(),
				row.get(row.column_index("slot").unwrap()).unwrap(),
				deco_skills,
			)));
		}
	}

	pub fn load_charms(&self, charms: &mut Charms, skills: &Skills) {
		let mut statement = self.connection.prepare(
			"SELECT charm.id, charm.previous_id, skilltree_id, level, name
FROM charm
JOIN charm_skill cs on charm.id = cs.charm_id
JOIN charm_text ct on charm.id = ct.id
WHERE lang_id = ?1").unwrap();
		let str = &*self.lang.borrow();
		let mut rows = statement.query(params![str]).unwrap();


		fn new_charm(row: &Row, skills: &Skills) -> Charm {
			let id = row.get(row.column_index("id").unwrap()).unwrap();
			let skill_id = row.get(row.column_index("skilltree_id").unwrap()).unwrap();
			let skill_lev = row.get(row.column_index("level").unwrap()).unwrap();
			let mut charm = Charm::new(
				id,
				row.get(row.column_index("name").unwrap()).unwrap(),
			);
			charm.add_skill(skills.get(&skill_id).unwrap(), skill_lev);
			charm
		}

		let row = rows.next().unwrap().unwrap();
		let mut charm = new_charm(row, &skills);
		let mut id = 0;
		while let Some(row) = rows.next().unwrap() {
			id = row.get(row.column_index("id").unwrap()).unwrap();
			if charm.id == id {
				let skill_id = row.get(row.column_index("skilltree_id").unwrap());
				let skill_lev = row.get(row.column_index("level").unwrap());
				charm.add_skill(skills.borrow().get(&skill_id.unwrap()).unwrap(), skill_lev.unwrap());
			} else {
				charms.borrow_mut().insert(charm.id, Arc::new(charm));
				charm = new_charm(row, &skills);
			}
		}
		charms.borrow_mut().insert(id, Arc::new(charm));
	}

	pub fn load_weapons(&self, weapons: &mut Weapons, skills: &Skills, setskills: &SetSkills) {
		let mut statement = self.connection.prepare(
			"SELECT weapon.id, previous_weapon_id, weapon_type, name,
		attack_true, affinity, sharpness, defense,
		slot_1, slot_2, slot_3,
		element1, element1_attack, element2, element2_attack, element_hidden, elderseal,
		armorset_bonus_id, skilltree_id
		FROM weapon
		LEFT JOIN weapon_skill ws ON weapon.id = ws.weapon_id
		LEFT JOIN weapon_text wt ON weapon.id = wt.id
		WHERE lang_id = ?1;").unwrap();
		let str = &*self.lang.borrow();
		let mut rows = statement.query(params![str]).unwrap();

		while let Some(row) = rows.next().unwrap() {
			let id = row.get(row.column_index("id").unwrap()).unwrap();
			let prev = row.get(row.column_index("previous_weapon_id").unwrap()).ok();
			let class = WeaponClass::from_str(row.get::<usize, String>(row.column_index("weapon_type").unwrap()).unwrap().as_str()).expect("Parse Error");
			let name = row.get_unwrap(row.column_index("name").unwrap());
			let affinity = row.get_unwrap(row.column_index("affinity").unwrap());
			let attack = row.get_unwrap(row.column_index("attack_true").unwrap());
			let defense = row.get(row.column_index("defense").unwrap()).unwrap();
			let slots = [row.get(row.column_index("slot_1").unwrap()).unwrap(),
				row.get(row.column_index("slot_2").unwrap()).unwrap(),
				row.get(row.column_index("slot_3").unwrap()).unwrap()];

			let sharpness = {
				let tmp = row.get(row.column_index("sharpness").unwrap());
				if let Ok(s) = tmp {
					let s: String = s;
					let mut sharp = [0u8; 7];
					let temp = s.as_str().split(',');
					for (i, n) in temp.enumerate() {
						sharp[i] = n.parse::<u8>().unwrap();
					}
					Some(sharp)
				} else { None }
			};

			let mut elements = Vec::new();

			if let Ok(e) = row.get::<usize, String>(row.column_index("element1").unwrap()) {
				elements.push((Element::from_str(e.to_lowercase().as_str()).expect("Parse error"), row.get_unwrap(row.column_index("element1_attack").unwrap())));
			}
			if let Ok(e) =  row.get::<usize, String>(row.column_index("element2").unwrap()) {
				elements.push((Element::from_str(e.to_lowercase().as_str()).expect("Parse error"), row.get_unwrap(row.column_index("element2_attack").unwrap())));
			}
			elements.shrink_to_fit();
			let element_hidden = row.get(row.column_index("element_hidden").unwrap()).unwrap();
			let elderseal = ElderSeal::new(row.get(row.column_index("element_hidden").unwrap()).ok());
			let armoset_bonus = {
				if let Some(id) = row.get(row.column_index("armorset_bonus_id").unwrap()).ok() {
					Some(Arc::clone(setskills.borrow().get(&id).unwrap()))
				} else { None }
			};
			let skill = {
				if let Some(id) = row.get(row.column_index("skilltree_id").unwrap()).ok() {
					Some(SkillLevel::new(Arc::clone(skills.borrow().get(&id).unwrap()), 1))
				} else { None }
			};

			let w = Weapon::new(id, prev, class, name,
				attack, affinity, sharpness, defense,
				slots, elements, element_hidden, elderseal,
				armoset_bonus, skill,
			);
			weapons.borrow_mut().insert(w.id, Arc::new(w));
		}
		weapons.borrow_mut().shrink_to_fit();
	}
}
