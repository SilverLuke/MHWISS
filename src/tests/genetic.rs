#![allow(unused_variables)]
use std::sync::Arc;
use crate::tests::Shared;
use crate::engines::hill_climbing::HillClimbing;
use crate::engines::Engine;
#[test]
#[ignore]
fn genetic() {
	println!("TEST: genetic");
	let shared = Shared::get();
	let storage = Arc::clone(&shared.storage);
	let constraints = Arc::clone(&shared.constraints);

	let mut engine = HillClimbing::new(storage, (*constraints).clone());
	let res = engine.run();
	println!("Result:\n{}", res.first().expect("No equipment found"));
	assert!(true);
}
