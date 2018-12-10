use super::*;

// This additional layer of indirection brought to you
// by the weird semi-support of Rust for enums.
// If we want to export it publicly under some name,
// we have to use this name in the initial declaration already.
pub enum Color {
	Black,
	White,
}

pub type Value = Color;

impl From<Value> for MessagePart {
	fn from(col: Value) -> MessagePart {
		let msg = match col {
			Color::Black => b"Black".to_vec(),
			Color::White => b"White".to_vec(),
		};
		MessagePart { msg }
	}
}

singleton_type!(Color);

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
			value!(
				Color::White,
				alt!(tag_no_case!("W") | tag_no_case!("white"))
			) |
			value!(
				Color::Black,
				alt!(tag_no_case!("B") | tag_no_case!("black"))
			)
		)
	}
}
