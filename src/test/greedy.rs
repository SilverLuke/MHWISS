use crate::engines::greedy::Greedy;
use crate::engines::{EnginesManager, Engine};
use crate::test::Shared;
use std::sync::Arc;

#[test]
fn greedy() {
	println!("TEST: greedy");
	let shared = Shared::get();
	let forge = &shared.forge;
	let searcher = EnginesManager::new(forge.clone());

	searcher.add_constraint(forge.get_skill_from_name("Artigiano").unwrap().id, 3);
	//searcher.add_constraint(forge.get_skill_from_name("Battitore").unwrap().id, 5);
	//engines.add_skill_requirement(datatypes.get_skill("Angelo custode").unwrap(), 1);
	//engines.add_skill_requirement(datatypes.get_skill("Critico elementale").unwrap(), 1);

	let mut engine = Greedy::new(Arc::clone(&forge), searcher.get_constrains());
	let res = engine.run();
	println!("Result:\n{}", res.first().expect("No equipment found"));
	assert!(true);
}