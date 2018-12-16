use super::super::messages::WriteGTP;
use super::*;
use std::io;

#[derive(Clone, Copy)]
pub enum Value {
	Black,
	White,
}

impl WriteGTP for Value {
	fn write_gtp(&self, f: &mut impl io::Write) -> io::Result<()> {
		match self {
			Value::Black => write!(f, "Black"),
			Value::White => write!(f, "White"),
		}
	}
}

singleton_type!(Color);

impl HasType<Type> for Value {
	fn has_type(&self, _t: &Type) -> bool {
		true
	}
}

impl Data for Value {
	type Type = Type;

	fn parse<'a, I: Input<'a>>(i: I, _t: &Self::Type) -> IResult<I, Self> {
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
