use std::rc::Rc;

use crate::data::db_storage::Storage;
use crate::data::dyn_storage::DynamicStorage;

pub(crate) struct CharmsPage {

}
#[allow(dead_code, unused)]
impl CharmsPage {
	pub fn new(builder: &gtk::Builder) -> CharmsPage {
		CharmsPage {
		}
	}

	pub fn show(&self, storage: &Rc<Storage>, dynamic_storage: &Rc<DynamicStorage>) {
		()
	}
}