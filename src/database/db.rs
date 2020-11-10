use rusqlite::{Connection, params, Row};
use std::collections::HashMap;
use crate::forge;
use std::rc::Rc;
use crate::forge::armor::{tr_armor_type, tr_rank};
use std::cell::{Cell, RefCell};
use std::borrow::{BorrowMut, Borrow};
use std::ops::Deref;

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

    pub fn load_set(&self, sets: &RefCell<HashMap<u16, forge::armor::Set>>, armors: &RefCell<HashMap<u16, Rc<forge::armor::Armor>>>) {
        let mut statement = self.connection.prepare(
            "SELECT armorset.id AS armorset_id, armor.id AS armor_id, armorset_text.name, armor.armor_type, armorset.rank
                FROM armor
                JOIN armorset ON armorset.id == armor.armorset_id
                JOIN armorset_text ON armorset_text.id == armorset.id
                WHERE armorset_text.lang_id = ?1").unwrap();
        let str = &*self.lang.borrow();
        let mut rows = statement.query(params![str]).unwrap();

        fn new_set(row: &Row, armors: &RefCell<HashMap<u16, Rc<forge::armor::Armor>>>) -> forge::armor::Set {
            let id = row.get(row.column_index("armorset_id").unwrap()).unwrap();
            let armor_id = row.get(row.column_index("armor_id").unwrap()).unwrap();
            let name = row.get(row.column_index("name").unwrap()).unwrap();
            let armor_type = tr_armor_type(row.get(row.column_index("armor_type").unwrap()).unwrap());
            let rank = tr_rank(row.get(row.column_index("rank").unwrap()).unwrap());
            let mut set = forge::armor::Set::new(id, name, rank);
            set.add_element(armor_type, armors.borrow().get(&armor_id).unwrap());
            set
        }

        let row = rows.next().unwrap().unwrap();
        let mut id = 0;
        let mut set = new_set(row, &armors);

        while let Some(row) = rows.next().unwrap() {
            id = row.get(row.column_index("armorset_id").unwrap()).unwrap();
            if set.id == id {
                let armor_id = row.get(row.column_index("armor_id").unwrap()).unwrap();
                let armor_type = tr_armor_type(row.get(row.column_index("armor_type").unwrap()).unwrap());
                set.add_element(armor_type, armors.borrow().get(&armor_id).unwrap());
            } else {
                sets.borrow_mut().insert(set.id, set);
                set = new_set(row, &armors);
            }
        }
        sets.borrow_mut().insert(id, set);
    }

    pub fn load_armors(&self, armors: &RefCell<HashMap<u16, Rc<forge::armor::Armor>>>, skills: &RefCell<HashMap<u16, Rc<forge::skill::Skill>>>) {
        let mut statement = self.connection.prepare(
            "SELECT armor.id, name, skilltree_id as skill, level, rank, armor_type, slot_1, slot_2, slot_3, defense_base, defense_max, defense_augment_max
FROM armor
LEFT JOIN armor_skill ON armor_skill.armor_id == armor.id
JOIN armor_text ON armor.id = armor_text.id
WHERE lang_id = ?1").unwrap();
        let str = &*self.lang.borrow();
        let mut rows = statement.query(params![str]).unwrap();

        fn new_armor(row: &Row, skills: &RefCell<HashMap<u16, Rc<forge::skill::Skill>>>) -> forge::armor::Armor {
            let id = row.get(row.column_index("id").unwrap()).unwrap();
            let skill_id = row.get(row.column_index("skill").unwrap());
            let skill_lev = row.get(row.column_index("level").unwrap());
            let mut armor = forge::armor::Armor::new(
                id,
                row.get(row.column_index("name").unwrap()).unwrap(),
                [row.get(row.column_index("slot_1").unwrap()).unwrap(),
                    row.get(row.column_index("slot_2").unwrap()).unwrap(),
                    row.get(row.column_index("slot_3").unwrap()).unwrap()],
                row.get(row.column_index("defense_base").unwrap()).unwrap(),
                row.get(row.column_index("defense_max").unwrap()).unwrap(),
                row.get(row.column_index("defense_augment_max").unwrap()).unwrap(),
            );
            if skill_id.is_ok() {
                armor.add_skill(skills.borrow_mut().get(&skill_id.unwrap()).unwrap(), skill_lev.unwrap());
            }
            armor
        }

        let row = rows.next().unwrap().unwrap();
        let mut armor = new_armor(row, &skills);
        let mut id = 0;
        while let Some(row) = rows.next().unwrap() {
            id = row.get(row.column_index("id").unwrap()).unwrap();
            if armor.id == id {
                let skill_id = row.get(row.column_index("skill").unwrap()).unwrap();
                let skill_lev = row.get(row.column_index("level").unwrap()).unwrap();
                armor.add_skill(skills.borrow().get(&skill_id).unwrap(), skill_lev);
            } else {
                armors.borrow_mut().insert(armor.id, Rc::new(armor));
                armor = new_armor(row, &skills);
            }
        }
        armors.borrow_mut().insert(id, Rc::new(armor));
    }

    pub fn load_skills(&self, skills: &RefCell<HashMap<u16, Rc<forge::skill::Skill>>>) {
        let mut statement = self.connection.prepare(
            "SELECT skilltree_text.id, name, skilltree_text.description, s.max_level
        FROM skilltree_text
        JOIN skilltree AS s ON skilltree_text.id = s.id
        WHERE skilltree_text.lang_id = ?1
        ORDER BY name").unwrap();
        let str = &*self.lang.borrow();
        let mut rows = statement.query(params![str]).unwrap();

        while let Some(row) = rows.next().unwrap() {
            let id = row.get(row.column_index("id").unwrap()).unwrap();
            skills.borrow_mut().insert(id, Rc::new(forge::skill::Skill::new(
                id,
                row.get(row.column_index("name").unwrap()).unwrap(),
                row.get(row.column_index("description").unwrap()).unwrap(),
                row.get(row.column_index("max_level").unwrap()).unwrap(),
            )));
        }
    }

    pub fn load_charms(&self, charms: &RefCell<HashMap<u16, Rc<forge::skill::Charm>>>, skills: &RefCell<HashMap<u16, Rc<forge::skill::Skill>>>) {
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
                row.get(row.column_index("name").unwrap()).unwrap()
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

    pub fn load_decorations(&self, deco: &RefCell<HashMap<u16, Rc<forge::skill::Decoration>>>, skills: &RefCell<HashMap<u16, Rc<forge::skill::Skill>>>) {
        let mut statement = self.connection.prepare(
            "SELECT decoration.id, name, slot, skilltree_id, skilltree_level, skilltree2_id, skilltree2_level
FROM decoration
JOIN decoration_text ON decoration_text.id == decoration.id
WHERE lang_id == ?1").unwrap();
        let str = &*self.lang.borrow();
        let mut rows = statement.query(params![str]).unwrap();

        while let Some(row) = rows.next().unwrap() {
            let skill1 = Some((
                skills.borrow().get(&row.get(row.column_index("skilltree_id").unwrap()).unwrap()).unwrap().clone(),
                row.get(row.column_index("skilltree_level").unwrap()).unwrap()
            ));
            let deco_skills = {
                let tmp = row.get(row.column_index("skilltree2_id").unwrap());
                if tmp.is_err() {
                    [skill1, None]
                } else {
                    let skill2 = Some((skills.borrow().get(&tmp.unwrap()).unwrap().clone(),
                                       row.get(row.column_index("skilltree2_level").unwrap()).unwrap()));
                    [skill1, skill2]
                }
            };
            let id = row.get(row.column_index("id").unwrap()).unwrap();
            deco.borrow_mut().insert(id, Rc::new(forge::skill::Decoration::new(
                id,
                row.get(row.column_index("name").unwrap()).unwrap(),
                deco_skills,
            )));
        }
    }
}

