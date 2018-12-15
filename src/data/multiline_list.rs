use super::*;

pub struct Value {
	data: Vec<one_liner::Value>,
}

pub type Type = one_liner::Type;

impl Typed for Value {
	type Type = Type;
}
