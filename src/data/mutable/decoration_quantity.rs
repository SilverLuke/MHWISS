use std::sync::Arc;
use crate::data::db_types::decoration::Decoration;

struct DecorationQuantiy {
	decoration: Arc<Decoration>,
	quantity: u32,
}