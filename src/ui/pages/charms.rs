use std::sync::Arc;
use crate::datatypes::forge::Forge;

pub(crate) struct CharmsPage {

}

impl CharmsPage {
	pub fn new(builder: &gtk::Builder) -> CharmsPage {
		CharmsPage {
		}
	}

	pub fn show(&self, forge: &Arc<Forge>) {
		()
	}
}