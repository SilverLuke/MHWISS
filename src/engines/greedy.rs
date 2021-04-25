use std::{
	fmt,
	cell::RefCell,
	ops::Not,
	rc::Rc,
	sync::Arc,
	cmp::Ordering,
	collections::{HashMap, hash_map::Entry},
	any::Any,
};
use crate::datatypes::{Level, ID,
	skill::SkillsLevel,
	forge::Forge,
	equipment::Equipment,
	weapon::Weapon,
	armor::Armor,
	charm::Charm,
	tool::Tool,
	decoration::{AttachedDecorations, Decoration},
	types::Item,
	types
};
use crate::engines::{EnginesManager, Engine};

type MagicValue = i16;

enum Wearable {
	Weapon(MagicValueContainer<Weapon>),
	Armor(MagicValueContainer<Armor>),
	Charm(MagicValueContainer<Charm>),
	Tool(MagicValueContainer<Tool>),
}

impl Wearable {
	fn get_value(&self) -> MagicValue {
		match self {
			Wearable::Weapon(i) => i.value,
			Wearable::Armor(i) => i.value,
			Wearable::Charm(i) => i.value,
			Wearable::Tool(i) => i.value,
		}
	}

	fn get_second_prop(&self) -> u16 {
		match self {
			Wearable::Weapon(i) => i.item.attack_true,
			Wearable::Armor(i) => i.item.defence[2] as u16,
			Wearable::Charm(i) => i.item.id,
			Wearable::Tool(i) => i.item.id,
		}
	}

	fn get_skills(&self) -> Option<HashMap<u16, u8>> {
		match self {
			Wearable::Weapon(i) => Some(i.item.get_skills_hash()),
			Wearable::Armor(i) => Some(i.item.get_skills_hash()),
			Wearable::Charm(i) => Some(i.item.get_skills_hash()),
			Wearable::Tool(i) => None,
		}
	}

	fn recalculate(&mut self, constrains: &HashMap<ID, Level>) {
		match self {
			Wearable::Weapon(item) => item.recalculate(constrains),
			Wearable::Armor(item) => item.recalculate(constrains),
			Wearable::Charm(item) => item.recalculate(constrains),
			Wearable::Tool(item) => item.recalculate(constrains),
		}
	}
}

impl Ord for Wearable {
	fn cmp(&self, other: &Self) -> Ordering {
		(self.get_value(), self.get_second_prop()).cmp(&(other.get_value(), other.get_second_prop()))
	}
}

impl PartialOrd for Wearable {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Eq for Wearable {}

impl PartialEq for Wearable {
	fn eq(&self, other: &Self) -> bool {
		(self.get_value(), self.get_second_prop()) == (other.get_value(), other.get_second_prop())
	}
}

struct MagicValueContainer<T: Item> {
	pub item: Arc<T>,
	pub value: MagicValue,
}

impl<T: Item> MagicValueContainer<T> {
	fn new(item: Arc<T>) -> Self {
		MagicValueContainer {
			item,
			value: 0,
		}
	}

	fn new_with_value(item: Arc<T>, constraint: &HashMap<ID, Level>) -> Self {
		let value = MagicValueContainer::<T>::skill_value(&item.get_skills_hash(), constraint);
		MagicValueContainer {
			item,
			value,
		}
	}

	fn with_slots(item: Arc<T>, constraint: &HashMap<ID, Level>, deco: &Vec<MagicValueContainer<Decoration>>) -> Self {
		let value = MagicValueContainer::<T>::skill_value(&item.get_skills_hash(), constraint);
		let deco_val = MagicValueContainer::<T>::decoration_value(item.get_slots(), deco);
		MagicValueContainer {
			item,
			value: value + deco_val,
		}
	}

	fn skill_value(item: &HashMap<ID, Level>, constraint: &HashMap<ID, Level>) -> MagicValue {
		let mut value = 0;
		for (id, val) in item {
			value += match constraint.get(id) {
				None => 0i16,
				Some(v) => *v as i16,
			};
		}
		value
	}

	fn decoration_value(slots: Option<Vec<u8>>, deco: &Vec<MagicValueContainer<Decoration>>) -> MagicValue {
		if let Some(vec) = slots {
			return 0;
			vec.len() as i16;
		}
		0
	}

	fn recalculate(&mut self, constrains: &HashMap<ID, Level>) {
		self.value = MagicValueContainer::<T>::skill_value(&self.item.get_skills_hash(), constrains);
	}

	fn get(&self) -> &Arc<T> {
		&self.item
	}
}

impl<T: Item> Ord for MagicValueContainer<T> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.value.cmp(&other.value)
	}
}

impl<T: Item> PartialOrd for MagicValueContainer<T> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl<T: Item> Eq for MagicValueContainer<T> {}

impl<T: Item> PartialEq for MagicValueContainer<T> {
	fn eq(&self, other: &Self) -> bool {
		self.value.eq(&other.value)
	}
}

pub(crate) struct Greedy {
	// Engine Related
	forge: Arc<Forge>,
	constrains: HashMap<ID, Level>,
	// Greedy related
	current_constrains: HashMap<ID, Level>,
	decorations: Vec<MagicValueContainer<Decoration>>,
	wearable: Vec<Wearable>,
}

impl Greedy {
	pub(crate) fn new(forge: Arc<Forge>, constrains: HashMap<ID, Level>) -> Self {
		let copy = constrains.clone();
		let mut decorations: Vec<MagicValueContainer<Decoration>> = Default::default();
		let mut wearable: Vec<Wearable> = Default::default();
		for decoration in forge.get_decorations_filtered(&copy).iter() {
			let container = MagicValueContainer::new_with_value(Arc::clone(decoration), &copy);
			decorations.push(container);
		}
		decorations.sort_by(|a, b| { b.cmp(&a) }); // Sorting descending

		for charm in forge.get_charms_filtered(&copy).iter() {
			let container = MagicValueContainer::new(Arc::clone(charm));
			wearable.push(Wearable::Charm(container));
		}
		for armor in forge.get_armors_filtered(&copy).iter() {
			let container = MagicValueContainer::with_slots(Arc::clone(armor), &copy, &decorations);
			wearable.push(Wearable::Armor(container));
		}
		for weapon in forge.get_weapons_filtered(&copy).iter() {
			let container = MagicValueContainer::with_slots(Arc::clone(weapon), &copy, &decorations);
			wearable.push(Wearable::Weapon(container));
		}
		// TODO add tools
		wearable.sort_by(|a, b| b.cmp(&a)); // Sorting descending

		Greedy {
			forge,
			constrains,
			current_constrains: copy,
			wearable,
			decorations,
		}
	}

	fn filter(&mut self) {
		for decoration in self.decorations.iter_mut() {
			decoration.recalculate(&self.current_constrains);
		}
		self.decorations.retain(|i| i.value > 0);
		self.decorations.sort_by(|a, b| { b.cmp(&a) }); // Sorting descending

		for w in self.wearable.iter_mut() {
			w.recalculate(&self.current_constrains);
		}
		self.wearable.retain(|i| i.get_value() > 0);
		self.wearable.sort_by(|a, b| { b.cmp(&a) }); // Sorting descending
		// TODO add tools
		self.wearable.sort_by(|a, b| b.cmp(&a)); // Sorting descending
	}

	fn remove_constrains(&mut self, skills: HashMap<ID, Level>) {
		for (id, val) in skills {
			match self.current_constrains.entry(id) {
				Entry::Occupied(mut o) => {
					let remaining: i16 = *o.get() as i16 - val as i16;
					if remaining <= 0 {
						dbg!(o.remove());
					} else {
						o.insert(remaining as u8);
					}
				}
				Entry::Vacant(v) => (), // println!("Skill {} not in requirements", self.forge.skills.get(&id).unwrap().name)
			};
		}
	}

	pub fn satisfy_constraint(&self, res: &Equipment) -> bool {
		let mut satisfied = true;
		let set_skills = res.get_skills();
		for (skill_id, constrains_level) in self.constrains.iter() {
			match set_skills.get(skill_id) {
				Some(equipment_level) => {
					if constrains_level > equipment_level {
						satisfied = false;
					}
				},
				None => satisfied = false,
			}
		}
		dbg!(satisfied)
	}
}

impl fmt::Debug for Greedy {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut str;
		str = format!("###    ARMORS   ###\n");
		/*
		for armors_list in self.armors.iter() {
			for elem in armors_list.iter() {
				str = format!("{} [{}] {}\n", str, elem.value, elem.item);
			}
		}
		*/
		str = format!("{}###    CHARMS   ###\n", str);
		/*
		for (charm, rank) in self.charms.iter() {
			str = format!("{} [{}] {}\n", str, rank, charm);
		}
		*/
		str = format!("{}###    DECORATIONS   ###\n", str);
		/*
		for (deco, rank) in self.decorations.iter() {
			str = format!("{} [{}] {}\n", str, rank, deco);
		}
		*/
		write!(f, "{}##################\n", str)
	}
}

impl Engine for Greedy {
	fn run(&mut self) -> Vec<Equipment> {
		let mut result = Equipment::new();
		let mut impossible = false;
		while self.satisfy_constraint(&result).not() && result.is_full().not() && impossible.not() {
			let mut i = 0;
			let mut inserted = false;
			while inserted.not() {  // Loop until a weareable item is suited for placement
				match self.wearable.get(i) {
					Some(piece) => {
						let result = match piece {
							Wearable::Weapon(item) => result.try_add_weapon(AttachedDecorations::new(Arc::clone(item.get()))),
							Wearable::Armor(item) => result.try_add_armor(AttachedDecorations::new(Arc::clone(item.get()))),
							Wearable::Charm(item) => result.try_add_charm(Arc::clone(item.get())),
							Wearable::Tool(item) => result.try_add_tool(AttachedDecorations::new(Arc::clone(item.get()))),
						}.is_ok();
						if result {
							if let Some(skills) = piece.get_skills() {
								self.remove_constrains(skills);
							}
							inserted = true;  // Go for the next piece
						} else {
							i += 1;
						}
					},
					None => {
						println!("IMPOSSIBLE");
						inserted = false;  // No more armor to be tested
						impossible = true;  // So no configuration exist (Maybe???)
					}
				};
			}
			self.filter();
		}
		vec![result]
	}
}