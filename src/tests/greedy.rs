use std::sync::Arc;
use crate::engines::{Engine, greedy::Greedy};
use crate::tests::Shared;

#[test]
fn greedy() {
	println!("TEST: greedy");
	let shared = Shared::get();
	let storage = Arc::clone(&shared.storage);
	let constraints = Arc::clone(&shared.constraints);

	let mut engine = Greedy::new(storage, constraints.as_ref().clone());
	let res = engine.run();
	println!("Result:\n{}", res.first().expect("No equipment found"));
	assert!(true);
}