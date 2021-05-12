use std::sync::Arc;
use crate::engines::{Engine, greedy::Greedy};
use crate::tests::Shared;

#[test]
fn greedy() {
	println!("TEST: greedy");
	let shared = Shared::get();
	let forge = Arc::clone(&shared.forge);
	let constrains = Arc::clone(&shared.constrains);

	let mut engine = Greedy::new(forge, constrains.as_ref().clone());
	let res = engine.run();
	println!("Result:\n{}", res.first().expect("No equipment found"));
	assert!(true);
}