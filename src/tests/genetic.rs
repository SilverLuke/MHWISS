#![allow(unused_variables, unused_imports)]
use std::sync::Arc;
use crate::tests::Shared;
use crate::engines::{Engine, hill_climbing::HillClimbing,};


#[test]
#[ignore]
fn genetic() {
	println!("TEST: genetic");
	let shared = Shared::get();
	let storage = Arc::clone(&shared.storage);
	/*let constraints = Arc::clone(&shared.constraints);

	let mut engine = HillClimbing::new(storage, (*constraints).clone());
	let res = engine.run();
	println!("Result:\n{}", res.first().expect("No equipment found")); */
	assert!(true);
}
