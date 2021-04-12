pub mod armor;
pub mod charm;
pub mod slots;
pub mod tool;
pub mod weapon;

pub trait UI<T> {  // Interface
	fn update(&self, piece: &Option<T>);

	fn empty(&self);

	fn show(&self, item: &T);
}