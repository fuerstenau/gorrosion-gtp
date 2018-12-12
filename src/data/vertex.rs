use super::*;

// This additional layer of indirection brought to you
// by the weird semi-support of Rust for enums.
// If we want to export it publicly under some name,
// we have to use this name in the initial declaration already.
/// A position where to play.
/// Either coordinates of a vertex on the board or `Pass`.
///
/// The spec says
/// > A vertex is a board coordinate
/// > consisting of one letter and one number
/// > [or `Pass`].
/// Vertices are not case sensitive.
/// Examples: “B13”, “j11”.
#[derive(Clone, Copy)]
pub enum Vertex {
	Pass,
	// TODO: Introduce types LetterCoord and NumberCoord?
	Coord(char, u8),
}

pub type Value = Vertex;

singleton_type!(Vertex);

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
		// Everything but “i” and “I”
		const LEGAL_LETTERS: & str =
			"abcdefghjklmnopqrstuvwxyzABCDEFGHJKLMNOPQRSTUVWXYZ";
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
