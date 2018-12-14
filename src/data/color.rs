use super::*;
use std::io;
use super::super::messages::Writable;

#[derive(Clone, Copy)]
pub enum Value {
	Black,
	White,
}

impl Writable for Value {
	fn write_gtp(&self, f: &mut impl io:: Write) -> io::Result<()> {
		match self {
			Value::Black => write!(f, "Black"),
			Value::White => write!(f, "White"),
		}
	}
}

singleton_type!(Color);

impl Typed for Value {
	type Type = Type;
}

impl HasType for Value {
	fn has_type(&self, _t: &Self::Type) -> bool {
		true
	}
}

impl Data for Value {
	fn parse<'a, I: Input<'a>>(i: I, _t: Self::Type) -> IResult<I, Self> {
		#[rustfmt::skip]
		alt!(i,
			value!(
				Value::White,
				alt!(tag_no_case!("W") | tag_no_case!("white"))
			) |
			value!(
				Value::Black,
				alt!(tag_no_case!("B") | tag_no_case!("black"))
			)
		)
	}
}
