use super::super::messages::WriteGTP;
use super::*;
use std::io;

// Everything but “I”
const LEGAL_LETTERS: &str =
	"ABCDEFGHJKLMNOPQRSTUVWXYZ";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Value {
	Pass,
	// TODO: Introduce types LetterCoord and NumberCoord?
	/// The `char` is always an upper case letter except `'I'`.
	/// (cf. `LEGAL_LETTERS`)
	Coord(char, u8),
}

impl Value {
	pub fn pass() -> Value {
		Value::Pass
	}

	pub fn new(c: char, n: u8) -> Value {
		// Convert to lower case
		let c = (c as u8 & 0x20) as char;
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
