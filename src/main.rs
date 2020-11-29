#![allow(dead_code, unused)]

mod ui;
mod forge;
mod database;

use crate::ui::ui::Ui;
use std::rc::Rc;


fn main() {
	let app = Ui::new();
	app.start(Rc::clone(&app));
}
