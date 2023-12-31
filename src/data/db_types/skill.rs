use std::{
	fmt,
	sync::Arc,
	cmp::Ordering,
	collections::{
		hash_map::Entry,
		HashMap,
	},
	hash::{
		Hash, Hasher
	},
};
use itertools::Itertools;
use crate::data::db_types::{ID, Level};


pub struct Skill {
	pub id: ID,
	pub name: String,
	pub description: String,
	pub max_level: u8,
	pub secret: u8,
	pub unlock: Option<Arc<Skill>>,
}

impl Skill {
	pub fn new(id: ID, name: String, description: String, max_level: u8, secret: u8, unlock: Option<Arc<Skill>>) -> Self {
		Skill { id, name, description, max_level, secret, unlock }
	}
}

impl PartialEq for Skill {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Eq for Skill {}

impl Hash for Skill {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.id.hash(state)
	}
}

impl fmt::Display for Skill {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} [{}]", self.name, self.id)
	}
}

// TODO Struct description
pub struct SetSkill {
	pub id: ID,
	pub name: String,
	pub skills: SkillsLevel,  // This level indicate the require level of self for enabling the skill related in skills
}

impl SetSkill {
	pub fn new(id: ID, name: String) -> Self {
		SetSkill { id, name, skills: SkillsLevel::new() }
	}

	pub fn add_skill(&mut self, skill: &Arc<Skill>, lev: u8) {
		self.skills.insert(SkillLevel::new(Arc::clone(skill), lev));
	}

	// Get the max required set skill for enable the skill
	pub fn get_max(&self) -> u8 {
		let mut max = 0;
		for i in self.skills.iter() {
			if i.level > max {
				max = i.level;
			}
		}
		max
	}
}

impl PartialEq for SetSkill {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Eq for SetSkill {}

impl Hash for SetSkill {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.id.hash(state)
	}
}

/*
SkillLevel is used to indicate the level/strength of the skill, generally used only in SkillsLevel
*/

pub struct SkillLevel {
	skill: Arc<Skill>,
	level: Level
}

impl SkillLevel {
	pub fn new(skill: Arc<Skill>, level: Level) -> Self {
		SkillLevel { skill, level }
	}

	pub fn get_id(&self) -> ID {
		self.skill.id
	}

	pub fn get_level(&self) -> Level {
		self.level
	}

	pub fn set_level(&mut self, new_level: Level) {
		self.level = new_level;
	}

	pub fn get_skill(&self) -> Arc<Skill> {
		Arc::clone(&self.skill)
	}
}

impl Clone for SkillLevel {
	fn clone(&self) -> Self {
		SkillLevel::new(Arc::clone(&self.skill), self.level)
	}
}

impl fmt::Display for SkillLevel {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} {}", self.skill.name, self.level)
	}
}

impl Eq for SkillLevel {}

impl PartialEq<Self> for SkillLevel {
	fn eq(&self, other: &Self) -> bool {
		self.skill.id != other.skill.id && self.level <= other.level
	}
}

impl PartialOrd<Self> for SkillLevel {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		if self.skill.id != other.skill.id {
			Some(self.level.cmp(&other.level))
		} else { None }
	}
}

impl Ord for SkillLevel {
	fn cmp(&self, other: &Self) -> Ordering {
		let ord = self.level.cmp(&other.level);
		if ord == Ordering::Equal {
			self.skill.name.cmp(&other.skill.name)
		} else {
			ord
		}
	}
}

impl Hash for SkillLevel {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.skill.hash(state)
	}
}

pub struct SkillsLevel {
	collection: HashMap<ID, SkillLevel>
}

impl SkillsLevel {
	pub fn new() -> Self {
		SkillsLevel {
			collection: Default::default(),
		}
	}

	// Insert in the hashmap the skill if not present, increase the skill level otherwise.
	pub fn insert(&mut self, add: SkillLevel) -> &Self {
		if add.get_level() <= 0 {
			self.collection.remove(&add.get_id());
		} else {
			match self.collection.entry(add.get_id()) {
				Entry::Occupied(mut o) => {
					let skill = o.get_mut();
					skill.set_level(skill.get_level() + add.level)
				},
				Entry::Vacant(v) => { v.insert(add); },
			};
		}
		self.shrink_to_fit();
		self
	}

	// Insert in the hashmap all the skills calling the insert() for each skill in add
	pub fn insert_skills(&mut self, add: &SkillsLevel) -> &Self {
		for (id, skill_to_add) in add.collection.iter() {
			match self.collection.entry(*id) {
				Entry::Occupied(mut o) => {
					let skill = o.get_mut();
					skill.set_level(skill.get_level() + skill_to_add.get_level())
				},
				Entry::Vacant(v) => { v.insert((*skill_to_add).clone()); }
			};
		}
		self.shrink_to_fit();
		self
	}

	// Insert in the hashmap the skill if not present, increase the skill level otherwise.
	pub fn remove(&mut self, remove: SkillLevel) -> Result<&Self, &str> {
		match self.collection.entry(remove.get_id()) {
			Entry::Occupied(mut o) => {
				let skill = o.get_mut();
				if skill.level > remove.level {
					skill.set_level(skill.level - remove.level);
				} else {
					o.remove_entry();
				}
				self.shrink_to_fit();
				Ok(self)
			},
			Entry::Vacant(_) => Err("Skill not found!")
		}
	}

	// Insert in the hashmap the skill if not present, increase the skill level otherwise.
	pub fn remove_skills(&mut self, remove_list: &SkillsLevel) {
		for (id, skill_to_remove) in remove_list.collection.iter() {
			match self.collection.entry(*id) {
				Entry::Occupied(mut o) => {
					let skill = o.get_mut();
					if skill.level > skill_to_remove.level {
						skill.set_level(skill.level - skill_to_remove.level);
					} else {
						o.remove_entry();
					}
				},
				Entry::Vacant(_) => {}
			};
		}
		self.shrink_to_fit();
	}

	pub fn set(&mut self, skill_to_add: SkillLevel) {
		if skill_to_add.get_level() <= 0 {
			self.collection.remove(&skill_to_add.get_id());
		} else {
			match self.collection.entry(skill_to_add.get_id()) {
				Entry::Occupied(mut o) => {
					let skill = o.get_mut();
					skill.set_level(skill_to_add.get_level())
				},
				Entry::Vacant(v) => { v.insert(skill_to_add); }
			};
		}
	}

	pub fn get_level(&self, skill: Arc<Skill>) -> Option<Level> {
		if let Some(skill_level) = self.collection.get(&skill.id) {
			Some(skill_level.get_level())
		} else { None }
	}

	pub fn contains_skill(&self, skill: Arc<Skill>) -> bool {
		self.collection.contains_key(&skill.id)
	}

	pub fn contains_id(&self, id: u16) -> bool {
		self.collection.contains_key(&id)
	}

	pub fn contains_list(&self, list: &SkillsLevel) -> bool {
		for id in list.collection.keys() {
			if self.collection.contains_key(id) {
				return true;
			}
		}
		false
	}

	pub fn iter(&self) -> Box<dyn Iterator<Item=&SkillLevel> + '_> {
		Box::new(self.collection.values())
	}

	pub fn shrink_to_fit(&mut self) {
		self.collection.shrink_to_fit();
	}

	pub fn len(&self) -> usize {
		self.collection.len()
	}
}

impl Clone for SkillsLevel {
	fn clone(&self) -> Self {
		SkillsLevel {
			collection: self.collection.clone()
		}
	}
}

impl fmt::Display for SkillsLevel {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.collection.is_empty() {
			return write!(f, "");
		}
		let mut str = String::from("");
		for skill in self.collection.values().sorted().rev() {
			if str.len() == 0 {
				str.push_str(skill.to_string().as_str());
			} else {
				str.push_str(format!(", {}", skill).as_str());
			}
		}
		write!(f, "<{}>", str)
	}
}

/*
//
//
// Structure helper for consuming iterator.
//
//
struct IntoIteratorHelper {
	iter: ::std::vec::IntoIter<SkillLevel>,
}

// implement the IntoIterator trait for a consuming iterator. Iteration will
// consume the Words structure
impl IntoIterator for SkillsLevel {
	type Item = SkillLevel;
	type IntoIter = IntoIteratorHelper;

	// note that into_iter() is consuming self
	fn into_iter(self) -> Self::IntoIter {
		let iter =  self.hashmap.into_iter().map(|(_k, v)| v).sorted();
		IntoIteratorHelper {
			iter,
		}
	}
}

// now, implements Iterator trait for the helper struct, to be used by adapters
impl Iterator for IntoIteratorHelper {
	type Item = SkillLevel;

	// just return the str reference
	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next()
	}
}

//
//
// Structure helper for non-consuming iterator.
//
//
struct IterHelper<'a> {
	iter: ::std::slice::Iter<'a, &'a SkillLevel>,
}

// implement the IntoIterator trait for a non-consuming iterator. Iteration will
// borrow the Words structure
impl<'a> IntoIterator for &'a SkillsLevel {
	type Item = &'a &'a SkillLevel;
	type IntoIter = IterHelper<'a>;

	// note that into_iter() is consuming self
	fn into_iter(self) -> Self::IntoIter {
		IterHelper {
			iter: self.hashmap.iter().map(|(_k, v)| v).collect_vec().iter(),
		}
	}
}

// now, implements Iterator trait for the helper struct, to be used by adapters
impl<'a> Iterator for IterHelper<'a> {
	type Item = &'a &'a SkillLevel;

	// just return the str reference
	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next()
	}
}

//
//
// structure helper for mutable non-consuming iterator.
//
//
struct IterMutHelper<'a> {
	iter: ::std::slice::IterMut<'a, SkillLevel>,
}

// implement the IntoIterator trait for a mutable non-consuming iterator. Iteration will
// mutably borrow the Words structure
impl<'a> IntoIterator for &'a mut SkillsLevel {
	type Item = &'a mut SkillLevel;
	type IntoIter = IterMutHelper<'a>;

	// note that into_iter() is consuming self
	fn into_iter(self) -> Self::IntoIter {
		let iter = self.hashmap.iter_mut().map(|(_k, v)| v).sorted();
		IterMutHelper {
			iter: iter.iter(),
		}
	}
}

// now, implements Iterator trait for the helper struct, to be used by adapters
impl<'a> Iterator for IterMutHelper<'a> {
	type Item = &'a mut SkillLevel;

	// just return the str reference
	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next()
	}
}
//
//
// implement FromIterator
//
//
use std::iter::FromIterator;
use itertools::Itertools;

impl FromIterator<SkillLevel> for SkillsLevel {
	fn from_iter<T>(iter: T) -> Self
		where
			T: IntoIterator<Item = SkillLevel> {

		// create and return Words structure
		SkillsLevel {
			hashmap: iter.into_iter().collect(),
		}
	}
}

impl<'a> SkillsLevel {
	fn iter(&'a self) -> IterHelper<'a> {
		self.into_iter()
	}

	fn iter_mut(&'a mut self) -> IterMutHelper<'a> {
		self.into_iter()
	}
}
*/
