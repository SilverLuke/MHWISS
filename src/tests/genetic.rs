#![allow(unused_variables)]
use std::sync::Arc;
use crate::tests::Shared;
use crate::engines::genetic::Genetic;
use crate::engines::Engine;
#[test]
#[ignore]
fn genetic() {
	println!("TEST: genetic");
	let shared = Shared::get();
	let forge = Arc::clone(&shared.forge);
	let constrains = Arc::clone(&shared.constrains);

	let mut engine = Genetic::new(forge, constrains.as_ref().clone());
	let res = engine.run();
	println!("Result:\n{}", res.first().expect("No equipment found"));
	assert!(true);
}
