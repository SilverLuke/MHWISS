use std::cell::RefCell;
use std::collections::{HashMap};
use std::sync::Arc;
use crate::data::{
	db_storage::Storage,
	db_types::{
		*,
		armor::{Armor, ArmorSet},
		charm::Charm,
		decoration::Decoration,
		skill::{SkillLevel, SkillsLevel},
		tool::Tool,
		weapon::Weapon,
	}
};

pub struct DynamicStorage {
	skills_constraints: RefCell<SkillsLevel>,
	quantity_decorations: RefCell<HashMap<Arc<Decoration>, u8>>,
	usable_weapons: RefCell<HashMap<Arc<Weapon>, bool>>,
	usable_armors:  RefCell<HashMap<Arc<Armor>, bool>>,
	usable_charms:  RefCell<HashMap<Arc<Charm>, bool>>,
	usable_tools:   RefCell<HashMap<Arc<Tool>, bool>>,
}

impl DynamicStorage {
	pub fn new(storage: &Storage) -> Self {
		let dynamic_storage =
			DynamicStorage {
				skills_constraints: RefCell::new(SkillsLevel::new()),
				quantity_decorations: Default::default(),
				usable_weapons: Default::default(),
				usable_armors: Default::default(),
				usable_charms: Default::default(),
				usable_tools: Default::default()
			};
		for decoration in storage.decorations.iter() {
			dynamic_storage.quantity_decorations.borrow_mut().insert(Arc::clone(decoration), 0);
		}
		for weapon in storage.weapons.iter() {
			dynamic_storage.usable_weapons.borrow_mut().insert(Arc::clone(weapon), true);
		}
		for armor in storage.armors.iter() {
			dynamic_storage.usable_armors.borrow_mut().insert(Arc::clone(armor), true);
		}
		for charm in storage.charms.iter() {
			dynamic_storage.usable_charms.borrow_mut().insert(Arc::clone(charm), true);
		}
		for tool in storage.tools.iter() {
			dynamic_storage.usable_tools.borrow_mut().insert(Arc::clone(tool), true);
		}
		dynamic_storage
	}

	pub fn set_constraint(&self, skill: SkillLevel) {
		self.skills_constraints.borrow_mut().set(skill);
	}
	pub fn clean_constrains(&self) {
		self.skills_constraints.replace(SkillsLevel::new());
	}
	pub fn get_constraints(&self) -> SkillsLevel {
		self.skills_constraints.borrow().clone()
	}

	pub fn set_decoration(&self, decoration: Arc<Decoration>, quantity: u8) {
		if let Some(val) = self.quantity_decorations.borrow_mut().get_mut(&decoration) {
			*val = quantity;
		} else {
			println!("Error no decoration found {}", decoration);
		}
	}

	pub fn set_weapon(&self, weapon: Arc<Weapon>, status: bool) {
		if let Some(val) = self.usable_weapons.borrow_mut().get_mut(&weapon) {
			*val = status;
		} else {
			println!("Error no weapon found")
		}
	}
	pub fn set_armors_set(&self, set_armor: Arc<ArmorSet>, status: bool) {
		for armor in set_armor.armors.iter() {
			if let Some(armor) = armor {
				self.set_armor(Arc::clone(armor), status);
			}
		}
	}
	pub fn set_armor(&self, armor: Arc<Armor>, status: bool) {
		if let Some(val) = self.usable_armors.borrow_mut().get_mut(&armor) {
			*val = status;
		} else {
			println!("Error no armor found")
		}
	}
	pub fn set_armors_by_rank(&self, rank: ArmorRank, status: bool) {
		for (armor, s) in self.usable_armors.borrow_mut().iter_mut() {
			if armor.rank == rank {
				*s = status;
			}
		}
	}
	pub fn set_charm(&self, charm: Arc<Charm>, status: bool) {
		if let Some(val) = self.usable_charms.borrow_mut().get_mut(&charm) {
			*val = status;
		} else {
			println!("Error no charm found")
		}
	}
	pub fn set_tool(&self, tool: Arc<Tool>, status: bool) {
		if let Some(val) = self.usable_tools.borrow_mut().get_mut(&tool) {
			*val = status;
		} else {
			println!("Error no charm found")
		}
	}
	// TODO return a new type of storage
	pub fn generate_storage(&self) -> Storage {
		let mut storage = Storage {
			skills: Default::default(),
			set_skills: Default::default(),
			armors: Default::default(),
			sets: Default::default(),
			decorations: Default::default(),
			charms: Default::default(),
			weapons: Default::default(),
			tools: Default::default()
		};
		// TODO Not used quantity
		for (decoration, quantity) in self.quantity_decorations.borrow().iter() {
			if *quantity > 0 {
				storage.decorations.insert(Arc::clone(decoration));
			}
		}
		for (armor, insert) in self.usable_armors.borrow().iter() {
			if *insert {
				storage.armors.insert(Arc::clone(armor));
			}
		}
		for (charm, insert) in self.usable_charms.borrow().iter() {
			if *insert {
				storage.charms.insert(Arc::clone(charm));
			}
		}
		for (tool, insert) in self.usable_tools.borrow().iter() {
			if *insert {
				storage.tools.insert(Arc::clone(tool));
			}
		}
		storage
	}
}