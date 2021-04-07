#![allow(dead_code, unused)]

use std::rc::Rc;
use std::sync::Arc;

use crate::datatypes::forge::Forge;
use crate::searcher::searcher::Searcher;
use crate::ui::ui::Ui;

mod ui;
mod datatypes;
mod searcher;
pub mod db;

fn main() {
	let mut forge = Forge::new();
	forge.load_all("it");
	let forge = Arc::new(forge);
	let searcher = Searcher::new(Arc::clone(&forge));
	let app = Ui::new(forge, searcher);
	app.start(Rc::clone(&app));
}
