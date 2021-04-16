#![allow(dead_code, unused)]

use std::rc::Rc;
use std::sync::Arc;

use crate::datatypes::forge::Forge;
use crate::engines::EnginesManager;
use crate::ui::ui::Ui;

mod ui;
mod datatypes;
mod engines;
mod db;

fn main() {
	let mut forge = Forge::new();
	forge.load_all("it");
	let forge = Arc::new(forge);
	let em = EnginesManager::new(Arc::clone(&forge));
	let app = Ui::new(forge, em);
	app.start();
}
