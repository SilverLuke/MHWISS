mod ui;
mod forge;
mod database;

use crate::ui::ui::Ui;
use std::rc::Rc;


fn main() {
	let mut app = Ui::new();
	app.start();
}
