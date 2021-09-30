use std::{
	cmp::Ordering,
	fmt,
	sync::Arc,
};
use std::collections::HashSet;
use std::ops::Not;
use crate::data::{
	db_storage::Storage,
	db_types::{
		HasSkills, HasDecoration, Slots,
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
use crate::engines::Engine;

type MagicValue = i16;
type Decos = Vec<EvalContainer<Arc<Decoration>>>;

trait Magicable {
	fn get_slots(&self) -> Slots;
}

impl Magicable for Arc<Charm> {
	fn get_slots(&self) -> Slots {
		vec![]
	}
}

impl<T> Magicable for AttachedDecorations<T> where T: HasDecoration {
	fn get_slots(&self) -> Slots {
		self.item.get_slots()
	}
}

impl Magicable for Arc<Decoration> {
	fn get_slots(&self) -> Slots {
		vec![]
	}
}

impl HasSkills for Arc<Charm> {  // ToDo is this required??
	fn get_skills(&self) -> SkillsLevel {
		Charm::get_skills(self)
	}
}

impl HasSkills for Arc<Decoration> {  // ToDo is this required??
	fn get_skills(&self) -> SkillsLevel {
		Decoration::get_skills(self)
	}
}


struct EvalContainer<T> {
	pub item: T,
	pub value: MagicValue,
}

fn eval_skills(item_skills: &SkillsLevel, constraint: &SkillsLevel) -> MagicValue {
	let mut value = 0;
	for skill in item_skills.iter() {
		value += match constraint.get_level(skill.get_skill()) {
			None => 0i16,
			Some(v) => v as i16,
		};
	}
	value
}
#[allow(unused_variables)]
fn eval_slots(slots: Slots, decorations: &Decos, constraint: &SkillsLevel) -> MagicValue {  // Sum of the value of best decorations applicable.
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

impl<T> EvalContainer<T> where T: Magicable + HasSkills {
	fn new(item: T, deco: Option<&Decos>, constraint: &SkillsLevel) -> Self {
		let mut value = eval_skills(&item.get_skills(), &constraint);
		if let Some(deco) = deco {
			value += eval_slots(item.get_slots(), deco, &constraint);
		}
		EvalContainer {
			item,
			value,
		}
	}

	pub fn evaluate(&mut self, deco: Option<&Decos>, constraints: &SkillsLevel) {
		let mut value = eval_skills(&self.item.get_skills(), &constraints);
		if let Some(deco) = deco {
			value += eval_slots(self.item.get_slots(), deco, &constraints);
		}
		self.value = value;
	}

	fn get(&self) -> T {
		todo!()
	}
}

enum Wearable {
	Weapon(EvalContainer<AttachedDecorations<Weapon>>),
	Armor(EvalContainer<AttachedDecorations<Armor>>),
	Charm(EvalContainer<Arc<Charm>>),
	Tool(EvalContainer<AttachedDecorations<Tool>>),
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

	fn get_skills(&self) -> SkillsLevel {
		match self {
			Wearable::Weapon(i) => i.item.get_skills(),
			Wearable::Armor(i) => i.item.get_skills(),
			Wearable::Charm(i) => i.item.get_skills(),
			Wearable::Tool(i) => i.item.get_skills(),
		}
	}

	fn recalculate(&mut self, constraint: &SkillsLevel, deco: &Decos) {
		match self {
			Wearable::Weapon(item) => item.evaluate(Some(deco), &constraint),
			Wearable::Armor(item) => item.evaluate(Some(deco), &constraint),
			Wearable::Charm(item) => item.evaluate(None, &constraint),
			Wearable::Tool(item) => item.evaluate(Some(deco), &constraint),
		}
	}
}

pub(crate) struct Greedy {
	// Engine Related
	//storage: Arc<Storage>,
	constraints: SkillsLevel,
	// Greedy related
	current_constrains: SkillsLevel,
	decorations: Decos,
	wearable: Vec<Wearable>,
}

fn filter_item<T>(items: &HashSet<Arc<T>>, constraints: &SkillsLevel) -> Vec<Arc<T>> where T: HasSkills {
	let mut ret = vec![];
	for item in items.iter() {
		if item.has_skills(constraints) {
			ret.push(Arc::clone(item));
		}
	}
	ret
}

impl Greedy {
	pub(crate) fn new(storage: Arc<Storage>, constraints: SkillsLevel) -> Self {
		let copy = constraints.clone();
		let mut decorations: Vec<EvalContainer<Arc<Decoration>>> = Default::default();
		let mut wearable: Vec<Wearable> = Default::default();

		for decoration in filter_item(&storage.decorations, &constraints).iter() {
			let container = EvalContainer::new(Arc::clone(decoration), None, &copy);
			decorations.push(container);
		}
		decorations.sort_by(|a, b| { b.cmp(&a) }); // Sorting descending

		for charm in filter_item(&storage.charms, &constraints).iter() {
			let container = EvalContainer::new(Arc::clone(charm), Some(&decorations), &copy);
			wearable.push(Wearable::Charm(container));
		}
		for armor in filter_item(&storage.armors, &constraints).iter() {
			let deco_conta = AttachedDecorations::new(Arc::clone(armor));
			let container = EvalContainer::new(deco_conta, Some(&decorations), &copy);  // TODO remove this Arc::new()
			wearable.push(Wearable::Armor(container));
		}
		for weapon in filter_item(&storage.weapons, &constraints).iter() {
			let deco_conta = AttachedDecorations::new(Arc::clone(weapon));
			let container = EvalContainer::new(deco_conta, Some(&decorations), &copy);
			wearable.push(Wearable::Weapon(container));
		}
		for tool in storage.tools.iter() {
			let deco_conta = AttachedDecorations::new(Arc::clone(tool));
			let container = EvalContainer::new(deco_conta, Some(&decorations), &copy);
			wearable.push(Wearable::Tool(container));
		}
		wearable.sort_by(|a, b| b.cmp(&a)); // Sorting descending

		Greedy {
			//storage,
			constraints,
			current_constrains: copy,
			wearable,
			decorations,
		}
	}

	fn filter(&mut self) {
		for decoration in self.decorations.iter_mut() {
			decoration.evaluate(None, &self.current_constrains);
		}
		self.decorations.retain(|i| i.value > 0);
		self.decorations.sort_by(|a, b| { b.cmp(&a) }); // Sorting descending

		for w in self.wearable.iter_mut() {
			w.recalculate(&self.current_constrains, &self.decorations);
		}
		self.wearable.retain(|i| i.get_value() > 0);
		self.wearable.sort_by(|a, b| { b.cmp(&a) });
	}

	pub fn satisfy_all_constraints(&self, res: &Equipment) -> bool {
		let mut satisfied = true;
		let equipment_skills = res.get_skills();
		for skill_level in self.constraints.iter() {
			match equipment_skills.get_level(skill_level.get_skill()) {
				Some(equipment_level) => {
					if skill_level.get_level() > equipment_level {
						satisfied = false;
					}
				},
				None => satisfied = false,
			}
		}
		satisfied
	}

	fn apply_best_decorations<T>(&self, item: &mut AttachedDecorations<T>) where T: HasDecoration {
		for deco in &self.decorations {
			let _ = item.try_add_deco(Arc::clone(&deco.item));
		}
	}
}

// TODO add how the engine works.
// FIXME Maybe the engine do not use charms

impl Engine for Greedy {
	fn run(&mut self) -> Vec<Equipment> {
		let mut result = Equipment::new();
		let mut impossible = false;
		while self.satisfy_all_constraints(&result).not() && result.is_full().not() && impossible.not() {
			let mut i = 0;
			let mut insered = false;
			while insered.not() {  // Loop until a weareable item is suited for placement
				match self.wearable.get(i) {
					Some(piece) => {
						let result = match piece {
							Wearable::Weapon(item) => {
								let mut weapon = item.get().clone();
								self.apply_best_decorations(&mut weapon);
								result.try_add_weapon(weapon)
							},
							Wearable::Armor(item) => {
								let mut armor = item.get().clone();
								self.apply_best_decorations(&mut armor);
								result.try_add_armor(armor)
							},
							Wearable::Tool(item) => {
								let mut tool = item.get().clone();
								self.apply_best_decorations(&mut tool);
								result.try_add_tool(tool)
							},
							Wearable::Charm(item) => result.try_add_charm(Arc::clone(&item.get())),
						}.is_ok();
						if result {
							self.current_constrains.remove_skills(&piece.get_skills());
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
