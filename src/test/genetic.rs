use std::sync::Arc;
use crate::engines::{EnginesManager, Engine, genetic::Genetic};
use crate::test::Shared;

#[test]
#[ignore]
fn genetic() {
	println!("TEST: genetic");
	let shared = Shared::get();
	let forge = &shared.forge;
	let searcher = EnginesManager::new(forge.clone());

	searcher.add_constraint(forge.get_skill_from_name("Artigiano").unwrap().id, 3);
	//engines.add_skill_requirement(datatypes.get_skill("Angelo custode").unwrap(), 1);
	//engines.add_skill_requirement(datatypes.get_skill("Critico elementale").unwrap(), 1);

	let mut engine = Genetic::new(Arc::clone(&forge), searcher.get_constrains());
	let res = engine.run();
	println!("Result:\n{}", res.first().expect("No equipment found"));
	assert!(true);
}


