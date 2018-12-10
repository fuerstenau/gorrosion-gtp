use super::*;
use nom::ParseTo;

// This additional layer of indirection brought to you
// by the weird semi-support of Rust for enums.
// If we want to export it publicly under some name,
// we have to use this name in the initial declaration already.
#[derive(Clone, Copy)]
pub enum Vertex {
	Pass,
	Coord(char, u8),
}

pub type Value = Vertex;

singleton_type!(Vertex);

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
		// Everything but “i” and “I”
		const LEGAL_LETTERS: &[Byte] =
			b"abcdefghjklmnopqrstuvwxyzABCDEFGHJKLMNOPQRSTUVWXYZ";
		#[rustfmt::skip]
		alt!(i,
			value!(Vertex::Pass, tag_no_case!("pass")) |
			do_parse!(
				letter: one_of!(LEGAL_LETTERS) >>
				digits: call!(nom::digit) >>
				(Vertex::Coord(
					letter,
					digits.parse_to().unwrap(),
				))
			)
		)
	}
}
