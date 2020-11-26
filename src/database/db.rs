use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::{Borrow, BorrowMut};
use gio::ListStoreExt;
use std::str;
use rusqlite::{Connection, params, Row};

use crate::forge;
use crate::forge::types::{Rank, ArmorClass, SetSkills, Gender, Element, WeaponClass, elder_seal, Weapons};
use crate::forge::skill::{Skill, SetSkill};
use crate::forge::types::{Skills, Armors, Sets, Decorations, Charms};
use crate::forge::weapon::Weapon;

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

	pub fn set_lang(&self, lang: String) {
		self.lang.replace(lang);
	}

	pub fn load_skills(&self, skills: &Skills) {
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
					Some(Rc::clone(skills.borrow().get(&unlock.unwrap()).unwrap()))
				} else {
					None
				}
			};
			let id = row.get(row.column_index("id").unwrap()).unwrap();
			let skill = forge::skill::Skill::new(
				id,
				row.get(row.column_index("name").unwrap()).unwrap(),
				row.get(row.column_index("description").unwrap()).unwrap(),
				row.get(row.column_index("max_level").unwrap()).unwrap(),
				row.get(row.column_index("secret").unwrap()).unwrap(),
				unlock
			);
			skills.borrow_mut().insert(id, Rc::new(skill));
		}
	}

	pub fn load_setskills(&self, setskills: &SetSkills, skills: &Skills) {
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
				setskills.borrow_mut().insert(setskill.id, Rc::new(setskill));
				setskill = new_setskill(row, skills);
			}
		}
		setskills.borrow_mut().insert(setskill.id, Rc::new(setskill));
	}

	pub fn load_armors(&self, armors: &Armors, skills: &Skills, setskills: &SetSkills) {
		let mut statement = self.connection.prepare(
			"SELECT armor.id, name, rank, armor_type,
					armorset_id, armorset_bonus_id,
					male, female,
					slot_1, slot_2, slot_3,
					defense_base, defense_max, defense_augment_max, fire, water, thunder, ice, dragon,
					skilltree_id, level
				FROM armor
					LEFT JOIN armor_skill ON armor_skill.armor_id == armor.id
					JOIN      armor_text ON armor.id = armor_text.id
				WHERE lang_id =  ?1;").unwrap();
		let str = &*self.lang.borrow();
		let mut rows = statement.query(params![str]).unwrap();

		fn new_armor(row: &Row, skills: &Skills, setskills: &SetSkills) -> forge::armor::Armor {
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
			let mut armor = forge::armor::Armor::new(
				row.get(row.column_index("id").unwrap()).unwrap(),
				row.get(row.column_index("name").unwrap()).unwrap(),
				ArmorClass::new(row.get(row.column_index("armor_type").unwrap()).unwrap()),
				gender,
				slots,
				defence,
				elements,
			);
			let skill_id = row.get(row.column_index("skilltree_id").unwrap());
			let skill_lev = row.get(row.column_index("level").unwrap());
			if skill_id.is_ok() {
				armor.add_skill(skills.borrow_mut().get(&skill_id.unwrap()).unwrap(), skill_lev.unwrap());
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
				armors.borrow_mut().insert(armor.id, Rc::new(armor));
				armor = new_armor(row, &skills, setskills);
			}
		}
		armor.skills.shrink_to_fit();
		armors.borrow_mut().insert(armor.id, Rc::new(armor));
	}

	pub fn load_sets(&self, sets: &Sets, armors: &Armors, setskills: &SetSkills) {
		let mut statement = self.connection.prepare(
			"SELECT armorset.id AS armorset_id, armor.id AS armor_id, armorset_text.name, armorset.rank, armor.armorset_bonus_id
FROM armorset
	 JOIN armor ON armorset.id == armor.armorset_id
	 JOIN armorset_text ON armorset_text.id == armorset.id
WHERE armorset_text.lang_id = ?1;").unwrap();
		let str = &*self.lang.borrow();
		let mut rows = statement.query(params![str]).unwrap();

		fn new_set(row: &Row, armors: &Armors, setskills: &SetSkills) -> forge::armor::Set {
			let id = row.get(row.column_index("armorset_id").unwrap()).unwrap();
			let armor_id = row.get(row.column_index("armor_id").unwrap()).unwrap();
			let name = row.get(row.column_index("name").unwrap()).unwrap();
			let rank = Rank::new(row.get(row.column_index("rank").unwrap()).unwrap());
			let skill = {
				if let Ok(id) = row.get(row.column_index("armorset_bonus_id").unwrap()) {
					Some(Rc::clone(setskills.borrow().get(&id).unwrap()))
				} else { None }
			};
			let mut set = forge::armor::Set::new(id, name, rank, skill);
			set.add_piece(armors.borrow().get(&armor_id).unwrap());
			set
		}

		let row = rows.next().unwrap().unwrap();
		let mut id = 0;
		let mut set = new_set(row, &armors, &setskills);

		while let Some(row) = rows.next().unwrap() {
			id = row.get(row.column_index("armorset_id").unwrap()).unwrap();
			if set.id == id {
				let armor_id = row.get(row.column_index("armor_id").unwrap()).unwrap();
				set.add_piece(armors.borrow().get(&armor_id).unwrap());
			} else {
				sets.borrow_mut().insert(set.id, Rc::new(set));
				set = new_set(row, &armors, &setskills);
			}
		}
		sets.borrow_mut().insert(id, Rc::new(set));
	}

	pub fn load_decorations(&self, deco: &Decorations, skills: &Skills) {
		let mut statement = self.connection.prepare(
			"SELECT decoration.id, name, slot, skilltree_id, skilltree_level, skilltree2_id, skilltree2_level
				 FROM decoration
				 JOIN decoration_text ON decoration_text.id == decoration.id
				 WHERE lang_id == ?1").unwrap();
		let str = &*self.lang.borrow();
		let mut rows = statement.query(params![str]).unwrap();

		while let Some(row) = rows.next().unwrap() {
			let mut deco_skills = Vec::with_capacity(1);
			let skill1: (Rc<Skill>, u8) = (skills.borrow().get(&row.get(row.column_index("skilltree_id").unwrap()).unwrap()).unwrap().clone(),
				row.get(row.column_index("skilltree_level").unwrap()).unwrap());
			deco_skills.push(skill1);
			let tmp = row.get(row.column_index("skilltree2_id").unwrap());
			if tmp.is_ok() {
				deco_skills.push((skills.borrow().get(&tmp.unwrap()).unwrap().clone(),
					row.get(row.column_index("skilltree2_level").unwrap()).unwrap()));
			}
			let id = row.get(row.column_index("id").unwrap()).unwrap();
			deco.borrow_mut().insert(id, Rc::new(forge::skill::Decoration::new(
				id,
				row.get(row.column_index("name").unwrap()).unwrap(),
				row.get(row.column_index("slot").unwrap()).unwrap(),
				deco_skills,
			)));
		}
	}

	pub fn load_charms(&self, charms: &Charms, skills: &Skills) {
		let mut statement = self.connection.prepare(
			"SELECT charm.id, charm.previous_id, skilltree_id, level, name
FROM charm
JOIN charm_skill cs on charm.id = cs.charm_id
JOIN charm_text ct on charm.id = ct.id
WHERE lang_id = ?1").unwrap();
		let str = &*self.lang.borrow();
		let mut rows = statement.query(params![str]).unwrap();


		fn new_charm(row: &Row, skills: &RefCell<HashMap<u16, Rc<forge::skill::Skill>>>) -> forge::skill::Charm {
			let id = row.get(row.column_index("id").unwrap()).unwrap();
			let skill_id = row.get(row.column_index("skilltree_id").unwrap()).unwrap();
			let skill_lev = row.get(row.column_index("level").unwrap()).unwrap();
			let mut charm = forge::skill::Charm::new(
				id,
				row.get(row.column_index("name").unwrap()).unwrap(),
			);
			charm.add_skill(skills.borrow_mut().get(&skill_id).unwrap(), skill_lev);
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
				charms.borrow_mut().insert(charm.id, Rc::new(charm));
				charm = new_charm(row, &skills);
			}
		}
		charms.borrow_mut().insert(id, Rc::new(charm));
	}

	pub fn load_weapons(&self, weapons: &Weapons, skills: &Skills, setskills: &SetSkills) {
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
			let class = WeaponClass::new(row.get(row.column_index("weapon_type").unwrap()).unwrap());
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
					let mut sharp = [0u8;7];
					let temp = s.as_str().split(',');
					for (i,n) in temp.enumerate() {
						sharp[i] = n.parse::<u8>().unwrap();
					}
					Some(sharp)
				} else {None}
			};

			let mut elements = Vec::new();

			if let Ok(e) = row.get(row.column_index("element1").unwrap()) {
				elements.push((Element::new(e), row.get_unwrap(row.column_index("element1_attack").unwrap())));
			}
			if let Ok(e) = row.get(row.column_index("element2").unwrap()) {
				elements.push((Element::new(e), row.get_unwrap(row.column_index("element2_attack").unwrap())));
			}
			let element_hidden = row.get(row.column_index("element_hidden").unwrap()).unwrap();
			let elderseal = elder_seal(row.get(row.column_index("element_hidden").unwrap()).ok());
			let armoset_bonus = {
				if let Some(id) = row.get(row.column_index("armorset_bonus_id").unwrap()).ok() {
					Some(Rc::clone(setskills.borrow().get(&id).unwrap()))
				} else { None }
			};
			let skill = {
				if let Some(id) = row.get(row.column_index("skilltree_id").unwrap()).ok() {
					Some(Rc::clone(skills.borrow().get(&id).unwrap()))
				} else { None }
			};

			let w = Weapon::new(id, prev, class, name,
				attack, affinity, sharpness, defense,
				slots, elements, element_hidden, elderseal,
				armoset_bonus, skill
			);
			weapons.borrow_mut().insert(w.id, Rc::new(w));
		}
		weapons.borrow_mut().shrink_to_fit();
	}
}



