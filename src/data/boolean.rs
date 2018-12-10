use super::*;

// This additional layer of indirection brought to you
// by the weird semi-support of Rust for enums.
// If we want to export it publicly under some name,
// we have to use this name in the initial declaration already.
pub enum Boolean {
	False,
	True,
}

pub type Value = Boolean;
singleton_type!(Boolean);

impl From<Value> for MessagePart {
	fn from(b: Value) -> MessagePart {
		let msg = match b {
			Boolean::False => b"false".to_vec(),
			Boolean::True => b"true".to_vec(),
		};
		MessagePart { msg }
	}
}

impl Data for Value {
	type Type = Type;

	fn parse(i: Input, _t: Self::Type) -> IResult<Input, Self> {
		#[rustfmt::skip]
		alt!(i,
			value!(Boolean::False, tag!("false")) |
			value!(Boolean::True, tag!("true"))
		)
	}

	fn typed(&self) -> Self::Type {
		Type::default()
	}
}
