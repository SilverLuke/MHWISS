use std::sync::Arc;
use crate::data::db_types::decoration::Decoration;

struct DecorationQuantity {
	decoration: Arc<Decoration>,
	quantity: u32,
}