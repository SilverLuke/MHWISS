#![allow(dead_code, unused)]

mod ui;
mod forge;
mod database;
mod searcher;

use crate::ui::ui::Ui;
use std::rc::Rc;
use crate::forge::forge::Forge;
use crate::searcher::searcher::Searcher;
use std::sync::Arc;


fn main() {
	let mut forge = Forge::new();
	forge.load_all("it");
	let forge = Arc::new(forge);
	let searcher = Searcher::new(Arc::clone(&forge));
	let app = Ui::new(forge, searcher);
	app.start(Rc::clone(&app));
}
