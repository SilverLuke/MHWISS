use std::{
	sync::Arc,
	cell::RefCell,
	collections::HashMap
};
use crate::datatypes::{
	*,
	types::Item,
	skill::Skill,
	armor::Armor,
	charm::Charm,
	decoration::Decoration,
};
use crate::db;

pub struct Forge {
	pub skills: Skills,  // Len 168
	pub set_skills: SetSkills,
	pub armors: Armors,
	pub sets: Sets,  // Len 343
	pub decorations: Decorations,
	pub charms: Charms,
	pub weapons: Weapons,
}

impl Forge {
	pub fn new() -> Self {
		Forge {
			skills: Default::default(),
			set_skills:Default::default(),
			armors: Default::default(),
			sets: Default::default(),
			decorations: Default::default(),
			charms: Default::default(),
			weapons: Default::default(),
		}
	}

	pub fn new_from(origin: Arc<Self>) -> Self {
		todo!()
	}

	pub fn get_skill_from_name(&self, name: &str) -> Option<Arc<Skill>> {
		for (_, skill) in self.skills.iter() {
			if skill.name == name {
				return Some(Arc::clone(skill));
			}
		}
		None
	}


	pub fn get_armors_filtered(&self, skills_req: &HashMap<ID, Level>) -> Vec<Arc<Armor>> {
		let mut ret: Vec<Arc<Armor>>  = Default::default();
		for (_id, armor) in self.armors.iter() {
			if armor.has_skills(&skills_req) {
				ret.push(Arc::clone(armor));
			}
		}
		ret.shrink_to_fit();
		ret
	}

	pub fn get_charms_filtered(&self, skills_req: &HashMap<ID, Level>) -> Vec<Arc<Charm>> {
		let mut ret  = vec![];
		for (_id, charm) in self.charms.iter() {
			if charm.has_skills(skills_req) {
				ret.push(Arc::clone(charm));
			}
		}
		ret
	}

	pub fn get_decorations_filtered(&self, skills_req: &HashMap<ID, Level>) -> Vec<Arc<Decoration>> {
		let mut ret  = vec![];
		for (_id, deco) in self.decorations.iter() {
			if deco.has_skills(skills_req) {
				ret.push(Arc::clone(deco));
			}
		}
		ret
	}

	pub fn load_all(&mut self, lang: &str) {
		let db = db::DB::new();
		db.set_lang(lang.to_string());
		db.load_skills(&mut self.skills);
		db.load_setskills(&mut self.set_skills, &self.skills);
		db.load_armors(&mut self.armors, &self.skills, &self.set_skills);
		db.load_sets(&mut self.sets, &self.armors, &self.set_skills);
		db.load_charms(&mut self.charms, &self.skills);
		db.load_decorations(&mut self.decorations, &self.skills);
		db.load_weapons(&mut self.weapons, &self.skills, &self.set_skills);
	}

	pub fn print_stat(&self) {
		println!("Loaded:");
		println!("\t{} skills", self.skills.len());
		println!("\t{} armorset skills", self.skills.len());
		println!("\t{} armors", self.armors.len());
		println!("\t{} sets", self.sets.len());
		println!("\t{} charms", self.charms.len());
		println!("\t{} decorations", self.decorations.len());
		println!("\t{} weapons", self.weapons.len());
	}
}