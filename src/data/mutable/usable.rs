use std::{
	sync::Arc,
	ops::Not,
};

struct Usable<T> {
	item: Arc<T>,
	usable: bool,
}

impl<T> Usable<T> {
	fn new(item: Arc<T>, usable: bool) -> Self {
		Usable{
			item,
			usable
		}
	}

	pub fn toggle_usable(&mut self) {
		self.usable = self.usable.not();
	}
}