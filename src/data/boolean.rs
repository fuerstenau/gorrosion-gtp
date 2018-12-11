use super::*;

// This additional layer of indirection brought to you
// by the weird semi-support of Rust for enums.
// If we want to export it publicly under some name,
// we have to use this name in the initial declaration already.
/// A boolean value, either `True` or `False`.
///
/// Most easily used, resp. provided, by casting to, resp. from, `bool`.
pub enum Boolean {
	False,
	True,
}

pub type Value = Boolean;

impl From<bool> for Value {
	fn from(b: bool) -> Value {
		if b {
			Boolean::True
		} else {
			Boolean::False
		}
	}
}

impl From<Value> for bool {
	fn from(v: Value) -> bool {
		match v {
			Boolean::True => true,
			Boolean::False => false,
		}
	}
}

singleton_type!(Boolean);

impl Typed for Value {
	type Type = Type;
}

impl HasType for Value {
	fn has_type(&self, _t: &Self::Type) -> bool {
		true
	}
}

impl Data for Value {
	fn parse(i: Input, _t: Self::Type) -> IResult<Input, Self> {
		#[rustfmt::skip]
		alt!(i,
			value!(Boolean::False, tag!("false")) |
			value!(Boolean::True, tag!("true"))
		)
	}
}
