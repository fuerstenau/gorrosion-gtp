use super::*;
use nom::ParseTo;

// This additional layer of indirection brought to you
// by the weird semi-support of Rust for enums.
// If we want to export it publicly under some name,
// we have to use this name in the initial declaration already.
pub enum Vertex {
	Pass,
	Coord(char, u8),
}

pub type Value = Vertex;

singleton_type!(Vertex);

impl From<Value> for MessagePart {
	fn from(vert: Vertex) -> MessagePart {
		let msg: Vec<Byte> = match vert {
			Vertex::Pass => b"pass".to_vec(),
			Vertex::Coord(letter, number) => {
				let mut num = Vec::from(
					number.to_string().as_bytes(),
				);
				let mut msg = Vec::with_capacity(num.len() + 1);
				msg.push(letter as Byte);
				msg.append(&mut num);
				msg
			}
		};
		MessagePart { msg }
	}
}

impl Data for Value {
	type Type = Type;

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

	fn typed(&self) -> Self::Type {
		Type::default()
	}
}
