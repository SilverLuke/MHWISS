use std::{
	str::FromStr,
	sync::Arc,
};
use rusqlite::{Connection, params, Row};
use crate::data::db_types::{
	*,
	charm::Charm,
	decoration::Decoration,
	skill::{SetSkill, Skill},
	ArmorClass, ArmorRank, ElderSeal, Element, Gender, WeaponClass,
	weapon::Weapon,
	armor::{Armor, ArmorSet},
	skill::{SkillLevel, SkillsLevel},
	tool::Tool,
};

pub struct DB {
	connection: rusqlite::Connection,
	lang: Option<String>,
}

impl DB {
	pub fn new() -> Self {
		let conn = Connection::open_with_flags("MHWorldData/mhw.db", rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();

		DB {
			connection: conn,
			lang: None,
		}
	}

	pub fn with_language(lang: String) -> Self {
		let conn = Connection::open_with_flags("MHWorldData/mhw.db", rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY).unwrap();

		DB {
			connection: conn,
			lang: Some(lang),
		}
	}

	pub fn get_available_languages(&self) -> Vec<(String, String)> {
		let mut statement = self.connection.prepare(
			"SELECT id, name FROM language;").unwrap();
		let mut rows = statement.query(params![]).unwrap();
		let mut ret = Vec::new();
		while let Some(row) = rows.next().unwrap() {
			let id: String = row.get("id").unwrap();
			let name: String = row.get("name").unwrap();
			ret.push((id, name));
		}
		ret
	}

	pub fn set_language(&mut self, lang: String) {
		self.lang = Some(lang);
	}

	pub fn load_skills(&self, skills: &mut Skills) {
		let mut statement = self.connection.prepare(
			"SELECT s.id, max_level, secret, unlocks_id, name, description
FROM skilltree AS s
JOIN skilltree_text ON skilltree_text.id = s.id
WHERE skilltree_text.lang_id = ?1
ORDER BY unlocks_id;").unwrap();
		let mut rows = statement.query(params![&self.lang]).unwrap();

		while let Some(row) = rows.next().unwrap() {
			let unlock_id = row.get("unlocks_id").unwrap();
			let unlock_skill = {
				if let Some(id) = unlock_id {
					Some(Arc::clone(get_skill_by_id(skills, id).unwrap()))
				} else {
					None
				}
			};
			let id = row.get("id").unwrap();
			let skill = Skill::new(
				id,
				row.get("name").unwrap(),
				row.get("description").unwrap(),
				row.get("max_level").unwrap(),
				row.get("secret").unwrap(),
				unlock_skill,
			);
			skills.insert(Arc::new(skill));
		}
	}

	pub fn load_set_skills(&self, setskills: &mut SetSkills, skills: &Skills) {
		let mut statement = self.connection.prepare(
			"SELECT setbonus_id, skilltree_id, required, name
		FROM armorset_bonus_skill AS abs
		JOIN armorset_bonus_text AS abt ON abs.setbonus_id = abt.id
		WHERE lang_id = ?1
		ORDER BY setbonus_id;").unwrap();
		let mut rows = statement.query(params![&self.lang]).unwrap();

		fn new_setskill(row: &Row, skills: &Skills) -> SetSkill {
			let id = row.get("setbonus_id").unwrap();
			let skill_id = row.get("skilltree_id").unwrap();
			let req = row.get("required").unwrap();
			let name = row.get("name").unwrap();
			let mut tmp = SetSkill::new(id, name);
			tmp.add_skill( get_skill_by_id(skills, skill_id).unwrap(), req);
			tmp
		}

		let row = rows.next().unwrap().unwrap();
		let mut id;
		let mut setskill = new_setskill(row, skills);

		while let Some(row) = rows.next().unwrap() {
			id = row.get("setbonus_id").unwrap();
			if setskill.id == id {
				let skill_id = row.get("skilltree_id").unwrap();
				let req = row.get("required").unwrap();
				setskill.add_skill(get_skill_by_id(skills, skill_id).unwrap(), req);
			} else {
				setskills.insert(Arc::new(setskill));
				setskill = new_setskill(row, skills);
			}
		}
		setskills.insert(Arc::new(setskill));
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
		let mut rows = statement.query(params![&self.lang]).unwrap();

		fn new_armor(row: &Row, skills: &Skills, setskills: &SetSkills) -> Armor {
			let slots = [row.get("slot_1").unwrap(),
				row.get("slot_2").unwrap(),
				row.get("slot_3").unwrap()];
			let defence = [row.get("defense_base").unwrap(),
				row.get("defense_max").unwrap(),
				row.get("defense_augment_max").unwrap()];
			let elements = [row.get("fire").unwrap(),
				row.get("water").unwrap(),
				row.get("thunder").unwrap(),
				row.get("ice").unwrap(),
				row.get("dragon").unwrap(),
			];
			let gender = Gender::new(row.get("male").unwrap(), row.get("female").unwrap());
			let str : String = row.get("armor_type").unwrap();
			let class = ArmorClass::from_str(&str).expect("Parse error");
			let str : String = row.get("rank").unwrap();
			let rank = ArmorRank::from_str(&str).expect("Parse error");
			let mut armor = Armor::new(
				row.get("id").unwrap(),
				row.get("name").unwrap(),
				class,
				rank,
				gender,
				slots,
				defence,
				elements,
			);
			let skill_id: Option<ID> = row.get("skilltree_id").unwrap();
			if let Some(id) = skill_id {
				let skill_lev = row.get("level").unwrap();
				armor.add_skill(get_skill_by_id(skills, id).unwrap(), skill_lev);
			}
			let setskill_id: Option<ID> = row.get("armorset_bonus_id").unwrap();
			if let Some(id) = setskill_id {
				armor.add_setskill(get_set_skill_by_id(setskills, id).unwrap());
			}
			armor
		}

		let row = rows.next().unwrap().unwrap();
		let mut armor = new_armor(row, &skills, setskills);
		let mut id;

		while let Some(row) = rows.next().unwrap() {
			id = row.get("id").unwrap();
			if armor.id == id {
				let skill_id = row.get("skilltree_id").unwrap();
				let skill_lev = row.get("level").unwrap();
				armor.add_skill(get_skill_by_id(skills, skill_id).unwrap(), skill_lev);
			} else {
				armor.skills.shrink_to_fit();
				armors.insert(Arc::new(armor));
				armor = new_armor(row, &skills, setskills);
			}
		}
		armor.skills.shrink_to_fit();
		armors.insert(Arc::new(armor));
	}

	pub fn load_sets(&self, sets: &mut ArmorSets, armors: &Armors, set_skills: &SetSkills) {
		let mut statement = self.connection.prepare(
			"SELECT armorset.id AS armorset_id, armor.id AS armor_id, armorset_text.name, armorset.rank, armor.armorset_bonus_id
FROM armorset
	 JOIN armor ON armorset.id == armor.armorset_id
	 JOIN armorset_text ON armorset_text.id == armorset.id
WHERE armorset_text.lang_id = ?1;").unwrap();
		let mut rows = statement.query(params![&self.lang]).unwrap();

		fn new_set(row: &Row, armors: &Armors, set_skills: &SetSkills) -> ArmorSet {
			let id = row.get("armorset_id").unwrap();
			let armor_id = row.get("armor_id").unwrap();
			let name = row.get("name").unwrap();
			let tmp:String = row.get("rank").unwrap();
			let rank: ArmorRank = ArmorRank::from_str(&tmp).expect("Parse Error");
			let skill = {
				if let Some(id) = row.get("armorset_bonus_id").unwrap() {
					Some(Arc::clone(get_set_skill_by_id(set_skills, id).unwrap()))
				} else { None }
			};
			let mut set = ArmorSet::new(id, name, rank, skill);
			set.add_armor(get_armor_by_id(armors, armor_id).unwrap());
			set
		}

		let row = rows.next().unwrap().unwrap();
		let mut id ;
		let mut set = new_set(row, &armors, &set_skills);

		while let Some(row) = rows.next().unwrap() {
			id = row.get("armorset_id").unwrap();
			if set.id == id {
				let armor_id = row.get("armor_id").unwrap();
				set.add_armor(get_armor_by_id(armors, armor_id).unwrap());
			} else {
				sets.insert( Arc::new(set));
				set = new_set(row, &armors, &set_skills);
			}
		}
		sets.insert(Arc::new(set));
	}

	pub fn load_decorations(&self, decorations: &mut Decorations, skills: &Skills) {
		let mut statement = self.connection.prepare(
			"SELECT decoration.id, name, slot, skilltree_id, skilltree_level, skilltree2_id, skilltree2_level
				 FROM decoration
				 JOIN decoration_text ON decoration_text.id == decoration.id
				 WHERE lang_id == ?1").unwrap();
		let mut rows = statement.query(params![&self.lang]).unwrap();

		while let Some(row) = rows.next().unwrap() {
			let mut deco_skills = SkillsLevel::new();

			let skill_id = row.get("skilltree_id").unwrap();
			let skill = get_skill_by_id(skills, skill_id).unwrap();
			let level = row.get("skilltree_level").unwrap();
			deco_skills.insert(SkillLevel::new(Arc::clone(skill), level));

			if let Some(skill_id) = row.get("skilltree2_id").unwrap() {
				let skill = get_skill_by_id(skills, skill_id).unwrap();
				let level = row.get("skilltree2_level").unwrap();
				deco_skills.insert(SkillLevel::new(Arc::clone(skill), level));
			}

			let id = row.get("id").unwrap();
			decorations.insert(Arc::new(Decoration::new(
				id,
				row.get("name").unwrap(),
				row.get("slot").unwrap(),
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
		let mut rows = statement.query(params![&self.lang]).unwrap();


		fn new_charm(row: &Row, skills: &Skills) -> Charm {
			let id = row.get("id").unwrap();
			let skill_id = row.get("skilltree_id").unwrap();
			let skill_lev = row.get("level").unwrap();
			let mut charm = Charm::new(
				id,
				row.get("name").unwrap(),
			);
			charm.add_skill(get_skill_by_id(skills, skill_id).unwrap(), skill_lev);
			charm
		}

		let row = rows.next().unwrap().unwrap();
		let mut charm = new_charm(row, &skills);
		let mut id ;
		while let Some(row) = rows.next().unwrap() {
			id = row.get("id").unwrap();
			if charm.id == id {
				let skill_id = row.get("skilltree_id").unwrap();
				let skill_lev = row.get("level").unwrap();
				charm.add_skill(get_skill_by_id(skills, skill_id).unwrap(), skill_lev);
			} else {
				charms.insert(Arc::new(charm));
				charm = new_charm(row, &skills);
			}
		}
		charms.insert( Arc::new(charm));
	}

	pub fn load_weapons(&self, weapons: &mut Weapons, skills: &Skills, set_skills: &SetSkills) {
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
		let mut rows = statement.query(params![&self.lang]).unwrap();

		while let Some(row) = rows.next().unwrap() {
			let id = row.get("id").unwrap();
			let prev = row.get("previous_weapon_id").unwrap();
			let tmp:String = row.get("weapon_type").unwrap();
			let class = WeaponClass::from_str(&tmp).expect("Parse Error");
			let name = row.get("name").unwrap();
			let affinity = row.get("affinity").unwrap();
			let attack = row.get("attack_true").unwrap();
			let defense = row.get("defense").unwrap();
			let slots = [row.get("slot_1").unwrap(),
				row.get("slot_2").unwrap(),
				row.get("slot_3").unwrap()];

			let sharpness = {
				let tmp: Option<String> = row.get("sharpness").unwrap();
				if let Some(s) = tmp {
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
			let tmp: Option<String> = row.get("element1").unwrap();
			if let Some(e) = tmp {
				elements.push((Element::from_str(e.to_lowercase().as_str()).expect("Parse error"), row.get("element1_attack").unwrap()));
			}
			let tmp: Option<String> = row.get("element2").unwrap();
			if let Some(e) = tmp {
				elements.push((Element::from_str(e.to_lowercase().as_str()).expect("Parse error"), row.get("element2_attack").unwrap()));
			}
			elements.shrink_to_fit();
			let element_hidden = row.get("element_hidden").unwrap();
			let tmp: Option<String> = row.get("elderseal").unwrap();
			let elderseal = ElderSeal::new(tmp);
			let armoset_bonus = {
				let tmp: Option<ID> = row.get("armorset_bonus_id").unwrap();
				if let Some(id) = tmp {
					Some(Arc::clone(get_set_skill_by_id(set_skills, id).unwrap()))
				} else { None }
			};
			let mut skill = SkillsLevel::new();
			if let Some(id) = row.get("skilltree_id").unwrap() {
				skill.insert(SkillLevel::new(Arc::clone(get_skill_by_id(skills, id).unwrap()), 1));
			}


			let w = Weapon::new(id, prev, class, name,
				attack, affinity, sharpness, defense,
				slots, elements, element_hidden, elderseal,
				armoset_bonus, skill,
			);
			weapons.insert(Arc::new(w));
		}
		weapons.shrink_to_fit();
	}

	pub fn load_tools(&self, tools: &mut Tools) {
		tools.insert(Arc::new(Tool::new(1, String::from("ToDo"), [2,2,1])));
	}
}

pub(crate) fn get_skill_by_id(skills: &Skills, id: ID) -> Option<&Arc<Skill>> {
	skills.iter().find(|s| s.id == id)
}

pub(crate) fn get_set_skill_by_id(set_skills: &SetSkills, id: ID) -> Option<&Arc<SetSkill>> {
	set_skills.iter().find(|s| s.id == id)
}

pub(crate) fn get_armor_by_id(armors: &Armors, id: ID) -> Option<&Arc<Armor>> {
	armors.iter().find(|s| s.id == id)
}

pub(crate) fn get_decorations_by_id(decorations: &Decorations, id: ID) -> Option<&Arc<Decoration>> {
	decorations.iter().find(|s| s.id == id)
}
