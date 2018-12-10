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

impl From<Value> for MessagePart {
	fn from(b: Value) -> MessagePart {
		let msg = match b {
			Boolean::False => b"false".to_vec(),
			Boolean::True => b"true".to_vec(),
		};
		MessagePart { msg }
	}
}

singleton_type!(Boolean);

impl Typed for Value {
	type Type = Type;
}

impl HasType for Value {
	fn has_type(&self, _t: Self::Type) -> bool {
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
