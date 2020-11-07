use rusqlite::{Connection, params, Row};
use std::collections::HashMap;
use crate::forge;
use std::rc::Rc;
use crate::forge::armor::{tr_armor_type, tr_rank};

pub struct DB {
    connection: rusqlite::Connection,
}

impl DB {
    pub fn new() -> Self {
        let conn = Connection::open("mhw.db").unwrap();
        DB {
            connection: conn
        }
    }

    pub fn load_set(&self, lang: &str, sets: &mut HashMap<u16, forge::armor::Set>, armors: &HashMap<u16, Rc<forge::armor::Armor>>) {
        let mut statement = self.connection.prepare(
            "SELECT armorset.id AS armorset_id, armor.id AS armor_id, armorset_text.name, armor.armor_type, armorset.rank
                FROM armor
                JOIN armorset ON armorset.id == armor.armorset_id
                JOIN armorset_text ON armorset_text.id == armorset.id
                WHERE armorset_text.lang_id = ?1").unwrap();
        let mut rows = statement.query(params![lang]).unwrap();


        let row = rows.next().unwrap().unwrap();

        fn new_set(row: &Row, armors: &HashMap<u16, Rc<forge::armor::Armor>>) -> forge::armor::Set {
            let id = row.get(row.column_index("armorset_id").unwrap()).unwrap();
            let armor_id = row.get(row.column_index("armor_id").unwrap()).unwrap();
            let name = row.get(row.column_index("name").unwrap()).unwrap();
            let armor_type = tr_armor_type(row.get(row.column_index("armor_type").unwrap()).unwrap());
            let rank = tr_rank(row.get(row.column_index("rank").unwrap()).unwrap());
            let mut set = forge::armor::Set::new(id, name, rank);
            set.add_element(armor_type, armors.get(&armor_id).unwrap());
            set
        }
        let mut id= 0;
        let mut set = new_set(row, armors);

        while let Some(row) = rows.next().unwrap() {
            id = row.get(row.column_index("armorset_id").unwrap()).unwrap();
            if set.id == id {
                let armor_id = row.get(row.column_index("armor_id").unwrap()).unwrap();
                let armor_type = tr_armor_type(row.get(row.column_index("armor_type").unwrap()).unwrap());
                set.add_element(armor_type, armors.get(&armor_id).unwrap());
            } else {
                sets.insert(set.id, set);
                set = new_set(row, armors);
            }
        }
        sets.insert(id, set);
    }

    pub fn load_armors(&self, lang: &str, armors: &mut HashMap<u16, Rc<forge::armor::Armor>>, skills: &HashMap<u16, Rc<forge::skill::Skill>>) {
        let mut statement = self.connection.prepare(
            "SELECT armor.id, name, skilltree_id as skill, level, rank, armor_type, slot_1, slot_2, slot_3, defense_base, defense_max, defense_augment_max
FROM armor
LEFT JOIN armor_skill ON armor_skill.armor_id == armor.id
JOIN armor_text ON armor.id = armor_text.id
WHERE lang_id = ?1").unwrap();
        let mut rows = statement.query(params![lang]).unwrap();
        fn new_armor(row: &Row, skills: &HashMap<u16, Rc<forge::skill::Skill>>) -> forge::armor::Armor {
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
                armor.add_skill(skills.get(&skill_id.unwrap()).unwrap(), skill_lev.unwrap());
            }
            armor
        }

        let row = rows.next().unwrap().unwrap();
        let mut armor = new_armor(row, skills);
        let mut id = 0;
        while let Some(row) = rows.next().unwrap() {
            id = row.get(row.column_index("id").unwrap()).unwrap();
            if armor.id == id {
                let skill_id = row.get(row.column_index("skill").unwrap());
                let skill_lev = row.get(row.column_index("level").unwrap());
                if skill_id.is_ok() {
                    armor.add_skill(skills.get(&skill_id.unwrap()).unwrap(), skill_lev.unwrap());
                }
            } else {
                armors.insert(armor.id, Rc::new(armor));
                armor = new_armor(row, skills);
            }
        }
        armors.insert(id, Rc::new(armor));
    }

    pub fn load_skills(&self, lang: &str, skills: &mut HashMap<u16, Rc<forge::skill::Skill>>) {
        let mut statement = self.connection.prepare(
            "SELECT skilltree_text.id, name, skilltree_text.description, s.max_level
        FROM skilltree_text
        JOIN skilltree AS s ON skilltree_text.id = s.id
        WHERE skilltree_text.lang_id = ?1
        ORDER BY name").unwrap();
        let mut rows = statement.query(params![lang]).unwrap();

        while let Some(row) = rows.next().unwrap() {
            let id = row.get(row.column_index("id").unwrap()).unwrap();
            skills.insert(id, Rc::new(forge::skill::Skill::new(
                id,
                row.get(row.column_index("name").unwrap()).unwrap(),
                row.get(row.column_index("description").unwrap()).unwrap(),
                row.get(row.column_index("max_level").unwrap()).unwrap(),
            )));
        }
    }
}

