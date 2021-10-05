use std::sync::Arc;
use crate::data::{
	db_storage::Storage,
	db_types::{
		Item,
		skill::SkillsLevel,
	}
};
use crate::engines::{EngineError::Impossible, Engine, greedy::Greedy};
use crate::tests::Shared;


#[test]
fn greedy_det() {
	println!("################################\nTEST: greedy deterministic\n################################");
	let shared = Shared::get();
	let storage = Arc::clone(&shared.storage);

	for constraints in shared.static_constraints.iter() {
		run(&storage, constraints.clone());
	}
	assert!(true);
}

#[test]
fn greedy_random() {
	println!("################################\nTEST: greedy random\n################################");
	let shared = Shared::get();
	let storage = Arc::clone(&shared.storage);
	for constraints in shared.random_constraints.iter() {
		run(&storage, constraints.clone());
	}
	assert!(true);
}

fn run(storage: &Arc<Storage>, constraints: SkillsLevel) {
	println!("Requirements:\n{}", constraints.to_string());
	let mut engine = Greedy::new(Arc::clone(storage), constraints);
	match engine.run() {
		Ok(result) => {
			let best = result.first().unwrap();
			println!("Set Skills:\n{}", best.get_skills());
			println!("Result:\n{}", best);
		},
		Err(e) => match e {
			Impossible => println!("Impossible"),
		}
	}
	println!("--------------------------------");
}