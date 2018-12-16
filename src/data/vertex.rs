use super::super::messages::WriteGTP;
use super::*;
use std::io;

// Everything but “i” and “I”
const LEGAL_LETTERS: &str =
	"abcdefghjklmnopqrstuvwxyzABCDEFGHJKLMNOPQRSTUVWXYZ";

#[derive(Clone, Copy)]
pub enum Value {
	Pass,
	// TODO: Introduce types LetterCoord and NumberCoord?
	Coord(char, u8),
}

impl Value {
	pub fn pass() -> Value {
		Value::Pass
	}

	pub fn new(c: char, n: u8) -> Value {
		assert!(LEGAL_LETTERS.contains(c));
		assert!((0 < n) && (n <= 25));
		Value::Coord(c, n)
	}
}

impl WriteGTP for Value {
	fn write_gtp(&self, f: &mut impl io::Write) -> io::Result<()> {
		match self {
			Value::Pass => write!(f, "pass"),
			Value::Coord(l, n) => {
				write!(f, "{}", l)?;
				write!(f, "{}", n)
			}
		}
	}
}

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
		#[rustfmt::skip]
		alt!(i,
			value!(Value::pass(), tag_no_case!("pass")) |
			do_parse!(
				letter: one_of!(LEGAL_LETTERS) >>
				digits: call!(nom::digit) >>
				(Value::new(
					letter,
					digits.parse_to().unwrap(),
				))
			)
		)
	}
}
