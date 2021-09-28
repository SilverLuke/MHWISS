use std::rc::Rc;
use std::sync::Arc;

use crate::data::{db::DB, db_storage::Storage};
use crate::engines::EnginesManager;
use crate::settings::Settings;
use crate::ui::Ui;

mod ui;
mod data;
mod engines;
mod settings;
#[cfg(test)]
mod tests;

fn main() {
	let settings = Settings::new();

	let db = DB::new(settings.get_language());

	let mut storage = Storage::new();
	storage.load_all(&db);
	let storage = Arc::new(storage);

	let em = Rc::new(EnginesManager::new(Arc::clone(&storage)));

	let app = Ui::new(settings, storage, em, db);
	app.start();
}