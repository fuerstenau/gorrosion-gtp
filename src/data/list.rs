use super::*;

pub struct Value {
	t: collection::Type,
	data: Vec<collection::Value>,
}

pub type Type = collection::Type;
