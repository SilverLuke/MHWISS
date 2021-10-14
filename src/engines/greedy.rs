use std::{
	cmp::{
		Ordering,
		min,
	},
	fmt,
	sync::Arc,
	ops::Not,
	collections::HashSet,
};
use crate::data::{
	db_storage::Storage,
	db_types::{
		Item, Decorations, Slot,
		weapon::Weapon,
		armor::Armor,
		charm::Charm,
		tool::Tool,
		skill::SkillsLevel,
		decoration::Decoration,
	},
	mutable::{
		equipment::Equipment,
		attached_decorations::AttachedDecorations,
	},
};
use crate::engines::{Engine, EngineError};

type EvalType = i16;

struct EvalContainer<T> {
	pub item: AttachedDecorations<T>,
	pub value: EvalType,
}

fn eval_skills(item_skills: &SkillsLevel, constraint: &SkillsLevel) -> EvalType {
	let mut value = 0;
	for skill in item_skills.iter() {
		value += match constraint.get_level(skill.get_skill()) {
			None => 0i16,
			Some(v) => min(v, skill.get_level()) as i16,
		};
	}
	value
}

fn eval_and_assign_slots<T>(item: &mut AttachedDecorations<T>, decorations: &Decorations, constraints: &mut SkillsLevel) -> EvalType where T: Item {  // Sum of the value of best decorations applicable.
	let mut val = 0;
	let slots = item.get_slots();
	if slots.len() <= 0 {
		return 0;  // The item has no slots
	}
	for slot in slots {
		let decoration = get_best_decoration(slot, decorations, constraints);
		if let Some(deco) = decoration {
			if item.try_add_deco(&deco) {
				let deco_skills = &deco.get_skills();
				val += eval_skills(deco_skills, constraints);
				constraints.remove_skills(deco_skills);
			}
		} else {
			break;
		}
	}
	val
}

fn get_best_decoration(slot_size: Slot, decorations: &Decorations, constraints: &SkillsLevel) -> Option<Arc<Decoration>> {
	let mut best: (EvalType, Option<&Arc<Decoration>>) = (0, None);
	for deco in decorations {
		if deco.size <= slot_size {
			let value = eval_skills(&deco.get_skills(), constraints);
			if best.1.is_some() {
				if value > best.0 || (value == best.0 && deco.get_skills().len() > best.1.unwrap().get_skills().len()) {
					best = (value, Some(deco));
				}
			} else {
				best = (value, Some(deco));
			}
		}
	}
	if let Some(deco) = best.1 {
		Some(Arc::clone(deco))
	} else {
		None
	}
}

impl<T: Item> EvalContainer<T> {
	fn new(item: &Arc<T>, deco: &Decorations, constraints: &SkillsLevel) -> Self {
		let item = AttachedDecorations::new(Arc::clone(item));
		let mut tmp = EvalContainer {
			item,
			value: 0i16,
		};
		tmp.evaluate(deco, constraints);
		tmp
	}

	pub fn evaluate(&mut self, deco: &Decorations, constraints: &SkillsLevel) {
		let mut value = eval_skills(&self.item.get_skills(), &constraints);
		if self.item.get_slots().len() > 0 {
			self.item.clean_decorations();
			let mut contraints_copy = constraints.clone();
			contraints_copy.remove_skills(&self.item.get_skills());
			value += eval_and_assign_slots(&mut self.item, deco, &mut contraints_copy);
		}
		self.value = value;
	}
}

enum Wearable {
	Weapon(EvalContainer<Weapon>),
	Armor(EvalContainer<Armor>),
	Charm(EvalContainer<Charm>),  // FIXME: In this way Charms are encapsulated inside attached_decorations, but charms cannot get decorations.
	Tool(EvalContainer<Tool>),
}

impl Wearable {
	fn get_value(&self) -> EvalType {
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
			Wearable::Charm(i) => i.item.get_skills().len() as u16,
			Wearable::Tool(i) => i.item.get_slots().len() as u16,
		}
	}

	fn get_skills(&self) -> SkillsLevel {
		match self {
			Wearable::Weapon(i) => i.item.get_skills(),
			Wearable::Armor(i) => i.item.get_skills(),
			Wearable::Charm(i) => i.item.get_skills(),
			Wearable::Tool(i) => i.item.get_skills(),
		}
	}

	fn recalculate(&mut self, constraint: &SkillsLevel, deco: &Decorations) {
		match self {
			Wearable::Weapon(item) => item.evaluate(deco, &constraint),
			Wearable::Armor(item) => item.evaluate(deco, &constraint),
			Wearable::Charm(item) => item.evaluate(deco,&constraint),
			Wearable::Tool(item) => item.evaluate(deco, &constraint),
		}
	}
}

pub(crate) struct Greedy {
	// Engine Related
	constraints: SkillsLevel,
	// Greedy related
	current_constrains: SkillsLevel,
	decorations: Decorations,
	wearable: Vec<Wearable>,
}

fn filter_item<T>(items: &HashSet<Arc<T>>, constraints: &SkillsLevel) -> Vec<Arc<T>> where T: Item {
	let mut ret = vec![];
	for item in items.iter() {
		if item.has_skills(constraints) {
			ret.push(Arc::clone(item));
		}
	}
	ret
}

impl Greedy {
	pub(crate) fn new(storage: Storage, constraints: SkillsLevel) -> Self {
		let copy = constraints.clone();
		let mut decorations: Decorations = Default::default();
		let mut wearable: Vec<Wearable> = Default::default();

		for decoration in filter_item(&storage.decorations, &constraints).iter() {
			decorations.insert(Arc::clone(decoration));
		}
		// decorations.sort_by(|a, b| { b.cmp(&a) }); Useless sorting. decorations value will change after first selected wearable



		for charm in storage.charms.iter() {
			let container = EvalContainer::new(charm, &decorations, &copy);
			wearable.push(Wearable::Charm(container));
		}
		for armor in storage.armors.iter() {
			let container = EvalContainer::new(armor, &decorations, &copy);
			wearable.push(Wearable::Armor(container));
		}
		for weapon in storage.weapons.iter() {
			let container = EvalContainer::new(weapon, &decorations, &copy);
			wearable.push(Wearable::Weapon(container));
		}

		/*
		for charm in filter_item(&storage.charms, &constraints).iter() {
			let container = EvalContainer::new(charm, &decorations, &copy);
			wearable.push(Wearable::Charm(container));
		}
		for armor in filter_item(&storage.armors, &constraints).iter() {
			let container = EvalContainer::new(armor, &decorations, &copy);
			wearable.push(Wearable::Armor(container));
		}
		for weapon in filter_item(&storage.weapons, &constraints).iter() {
			let container = EvalContainer::new(weapon, &decorations, &copy);
			wearable.push(Wearable::Weapon(container));
		}*/
		for tool in storage.tools.iter() {
			let container = EvalContainer::new(tool, &decorations, &copy);
			wearable.push(Wearable::Tool(container));
		}
		wearable.sort_by(|a, b| b.cmp(&a)); // Sorting descending

		Greedy {
			constraints,
			current_constrains: copy,
			wearable,
			decorations,
		}
	}

	fn filter(&mut self) {
		let constrains = &self.current_constrains;
		self.decorations.retain(|decoration| { eval_skills(&decoration.get_skills(), &constrains) > 0 });

		for w in self.wearable.iter_mut() {
			w.recalculate(&self.current_constrains, &self.decorations);
		}
		self.wearable.retain(|i| i.get_value() > 0);
		self.wearable.sort_by(|a, b| { b.cmp(&a) });
	}

	pub fn satisfy_all_constraints(&self, res: &Equipment) -> bool {
		let mut satisfied = true;
		let equipment_skills = res.get_skills();
		for constraint_skill in self.constraints.iter() {
			match equipment_skills.get_level(constraint_skill.get_skill()) {
				Some(equipment_level) => {
					if constraint_skill.get_level() > equipment_level {
						satisfied = false;
					}
				},
				None => satisfied = false,
			}
		}
		satisfied
	}
}

impl Engine for Greedy {
	fn run(&mut self) -> Result<Vec<Equipment>, EngineError> {
		let mut result = Equipment::new();
		while self.satisfy_all_constraints(&result).not() && result.is_full().not() {
			let mut i = 0;
			let mut insered = false;
			while insered.not() {  // Loop until a wearable item is suited for placement
				match self.wearable.get(i) {
					Some(piece) => {
						insered = match piece {
							Wearable::Weapon(item) => {
								let weapon = item.item.clone();
								result.try_add_weapon(weapon)
							},
							Wearable::Armor(item) => {
								let armor = item.item.clone();
								result.try_add_armor(armor)
							},
							Wearable::Charm(item) => {
								let charm = Arc::clone(&item.item.item);
								result.try_add_charm(charm)
							},
							Wearable::Tool(item) => {
								let tool = item.item.clone();
								result.try_add_tool(tool)
							},
						};
						if insered {  // Go for the next piece
							self.current_constrains.remove_skills(&piece.get_skills());
						} else {
							i += 1;
						}
					},
					None => return Err(EngineError::Impossible),
				};
			}
			self.filter();
		}
		Ok(vec![result])
	}
}

impl<T> Ord for EvalContainer<T> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.value.cmp(&other.value)
	}
}

impl<T> PartialOrd for EvalContainer<T> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl<T> Eq for EvalContainer<T> {}

impl<T> PartialEq for EvalContainer<T> {
	fn eq(&self, other: &Self) -> bool {
		self.value.eq(&other.value)
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
