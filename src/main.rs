use std::rc::Rc;
use std::sync::Arc;

use crate::db::DB;
use crate::datatypes::forge::Forge;
use crate::engines::EnginesManager;
use crate::ui::Ui;
use crate::settings::Settings;

mod ui;
mod datatypes;
mod engines;
mod db;
mod settings;
#[cfg(test)]
mod tests;

fn main() {
	let settings = Settings::new();

	let db = DB::new(settings.get_language());

	let mut forge = Forge::new();
	forge.load_all(&db);
	let forge = Arc::new(forge);

	let em = Rc::new(EnginesManager::new(Arc::clone(&forge)));

	let app = Ui::new(settings, forge, em, db);
	app.start();
}