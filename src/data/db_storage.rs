use crate::data::{
	db::DB,
	db_types::{Weapons, Charms, Decorations, Sets, Armors, SetSkills, Skills, Tools},
};

pub struct Storage {
	pub skills: Skills,  // Len 168
	pub set_skills: SetSkills,
	pub armors: Armors,
	pub sets: Sets,  // Len 343
	pub decorations: Decorations,
	pub charms: Charms,
	pub weapons: Weapons,
	pub tools: Tools,
}

impl Storage {
	pub fn new() -> Self {
		Storage {
			skills: Default::default(),
			set_skills:Default::default(),
			armors: Default::default(),
			sets: Default::default(),
			decorations: Default::default(),
			charms: Default::default(),
			weapons: Default::default(),
			tools: Default::default(),
		}
	}

	pub fn load_all(&mut self, db: &DB) {
		db.load_skills(&mut self.skills);
		db.load_setskills(&mut self.set_skills, &self.skills);
		db.load_armors(&mut self.armors, &self.skills, &self.set_skills);
		db.load_sets(&mut self.sets, &self.armors, &self.set_skills);
		db.load_charms(&mut self.charms, &self.skills);
		db.load_decorations(&mut self.decorations, &self.skills);
		db.load_weapons(&mut self.weapons, &self.skills, &self.set_skills);
		db.load_tools(&mut self.tools);
	}

	#[allow(dead_code)]
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