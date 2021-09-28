use std::sync::Arc;

use crate::data::db_storage::Storage;

pub(crate) struct CharmsPage {

}
#[allow(dead_code, unused)]
impl CharmsPage {
	pub fn new(builder: &gtk::Builder) -> CharmsPage {
		CharmsPage {
		}
	}

	pub fn show(&self, storage: &Arc<Storage>) {
		()
	}
}