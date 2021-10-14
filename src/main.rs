use crate::data::{db::DB, db_storage::Storage};
use crate::settings::Settings;
use crate::ui::Ui;

mod ui;
mod data;
mod engines;
mod settings;
#[cfg(test)]
mod tests;

fn main() {
	let mut db = DB::new();
	let settings = Settings::new(&db);
	db.set_language(settings.get_language());

	let mut storage = Storage::new();
	storage.load_all(&db);
//	let storage = Rc::new(storage);

	let app = Ui::new(settings, storage);
	app.start();
}