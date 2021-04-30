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
use crate::datatypes::types::Decorable;

type MagicValue = i16;

enum Wearable {
	Weapon(MagicValueContainer<AttachedDecorations<Weapon>>),
	Armor(MagicValueContainer<AttachedDecorations<Armor>>),
	Charm(MagicValueContainer<Charm>),
	Tool(MagicValueContainer<AttachedDecorations<Tool>>),
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
			Wearable::Weapon(i) => i.item.item.attack_true,
			Wearable::Armor(i) => i.item.item.defence[2] as u16,
			Wearable::Charm(i) => i.item.id,
			Wearable::Tool(i) => i.item.item.id,
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

	fn recalculate(&mut self, constrains: &HashMap<ID, Level>, deco: &Vec<MagicValueContainer<Decoration>>) {
		match self {
			Wearable::Weapon(item) => item.recalculate_slots(constrains, deco),
			Wearable::Armor(item) => item.recalculate_slots(constrains, deco),
			Wearable::Charm(item) => item.recalculate(constrains),
			Wearable::Tool(item) => item.recalculate_slots(constrains, deco),
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
	fn new(item: Arc<T>, constraint: &HashMap<ID, Level>) -> Self {
		let value = MagicValueContainer::<T>::skill_value(&item.get_skills_hash(), constraint);
		MagicValueContainer {
			item,
			value,
		}
	}

	fn with_slots(item: Arc<T>, constraint: &HashMap<ID, Level>, deco: &Vec<MagicValueContainer<Decoration>>) -> Self {
		let value = MagicValueContainer::<T>::skill_value(&item.get_skills_hash(), constraint);
		let deco_val = MagicValueContainer::<T>::decoration_value(item.get_slots().unwrap(), deco);
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
				Some(v) => *v as MagicValue,
			};
		}
		value
	}

	fn decoration_value(slots: Vec<u8>, decorations: &Vec<MagicValueContainer<Decoration>>) -> MagicValue {  // Sum of the value of best decorations applicable.
		let mut val = 0;
		let mut i = 0;
		for deco in decorations {
			if let Some(slot_size) = slots.get(i) {
				if deco.item.size == *slot_size {
					i += 1;
					val += deco.value;
				}
			} else {
				break;
			}
		}
		val
	}

	fn recalculate(&mut self, constrains: &HashMap<ID, Level>) {
		self.value = MagicValueContainer::<T>::skill_value(&self.item.get_skills_hash(), constrains);
	}

	fn recalculate_slots(&mut self, constrains: &HashMap<ID, Level>, deco: &Vec<MagicValueContainer<Decoration>>) {
		let skill_val = MagicValueContainer::<T>::skill_value(&self.item.get_skills_hash(), constrains);
		let deco_val = if let Some(slots) = self.item.get_slots() {
			MagicValueContainer::<T>::decoration_value(slots, deco)
		} else { 0 };
		self.value = skill_val + deco_val;
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
			let container = MagicValueContainer::new(Arc::clone(decoration), &copy);
			decorations.push(container);
		}
		decorations.sort_by(|a, b| { b.cmp(&a) }); // Sorting descending

		for charm in forge.get_charms_filtered(&copy).iter() {
			let container = MagicValueContainer::new(Arc::clone(charm), &copy);
			wearable.push(Wearable::Charm(container));
		}
		for armor in forge.get_armors_filtered(&copy).iter() {
			let deco_conta = AttachedDecorations::new(Arc::clone(armor));
			let container = MagicValueContainer::with_slots(Arc::new(deco_conta), &copy, &decorations);  // TODO remove this Arc::new()
			wearable.push(Wearable::Armor(container));
		}
		for weapon in forge.get_weapons_filtered(&copy).iter() {
			let deco_conta = AttachedDecorations::new(Arc::clone(weapon));
			let container = MagicValueContainer::with_slots(Arc::new(deco_conta), &copy, &decorations);
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
			w.recalculate(&self.current_constrains, &self.decorations);
		}
		self.wearable.retain(|i| i.get_value() > 0);
		self.wearable.sort_by(|a, b| { b.cmp(&a) });
	}

	fn remove_constrains(&mut self, skills: HashMap<ID, Level>) {
		for (id, val) in skills {
			match self.current_constrains.entry(id) {
				Entry::Occupied(mut o) => {
					let remaining: i16 = *o.get() as i16 - val as i16;
					if remaining <= 0 {
						o.remove();
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
		satisfied
	}

	fn apply_best_decorations<T: Item + Decorable>(&self, item: &mut AttachedDecorations<T>) {
		let mut i = 0;
		for deco in &self.decorations {
			item.try_add_deco(Arc::clone(&deco.item));
		}
	}
}

impl Engine for Greedy {
	fn run(&mut self) -> Vec<Equipment> {
		let mut result = Equipment::new();
		let mut impossible = false;
		while self.satisfy_constraint(&result).not() && result.is_full().not() && impossible.not() {
			let mut i = 0;
			let mut insered = false;
			while insered.not() {  // Loop until a weareable item is suited for placement
				match self.wearable.get(i) {
					Some(piece) => {
						let result = match piece {
							Wearable::Weapon(item) => {
								let mut weapon = (**item.get()).clone();
								self.apply_best_decorations(&mut weapon);
								result.try_add_weapon(weapon)
							},
							Wearable::Armor(item) => {
								let mut armor = (**item.get()).clone();
								self.apply_best_decorations(&mut armor);
								result.try_add_armor(armor)
							},
							Wearable::Tool(item) => {
								let mut tool = (**item.get()).clone();
								self.apply_best_decorations(&mut tool);
								result.try_add_tool(tool)
							},
							Wearable::Charm(item) => result.try_add_charm(Arc::clone(item.get())),
						}.is_ok();
						if result {
							if let Some(skills) = piece.get_skills() {
								self.remove_constrains(skills);
							}
							insered = true;  // Go for the next piece
						} else {
							i += 1;
						}
					},
					None => {
						println!("IMPOSSIBLE");
						insered = true;  // No more armor to be tested
						impossible = true;  // So no configuration exist (Maybe???)
					}
				};
			}
			self.filter();
		}
		vec![result]
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
