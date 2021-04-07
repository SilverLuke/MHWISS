pub mod armor;
pub mod charm;
pub mod slots;
pub mod tool;
pub mod weapon;

pub trait UI<T> {  // Interface
	fn update(&self, piece: &Option<T>);

	fn set_empty(&self);

	fn show_item(&self, item: &T);
}