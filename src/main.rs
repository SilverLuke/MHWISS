#![allow(dead_code, unused)]

use std::rc::Rc;
use std::sync::Arc;

use crate::datatypes::forge::Forge;
use crate::engines::EnginesManager;
use crate::ui::ui::Ui;
use crate::db::DB;

mod ui;
mod datatypes;
mod engines;
mod db;
#[cfg(test)]
mod test;

fn main() {
	let mut forge = Forge::new();
	let db = DB::new();
	db.set_lang("it".to_string());
	forge.load_all(db);
	let forge = Arc::new(forge);
	let em = Rc::new(EnginesManager::new(Arc::clone(&forge)));
	let app = Ui::new(forge, em);
	app.start();
}