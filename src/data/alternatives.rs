//! The so-called specification is unclear
//! on the matter of alternatives of compound types.
//! For now, we'll do the simpler thing.

use super::super::messages::WriteGTP;
use super::*;
use std::io;

pub struct Value(simple_entity::Value);

impl From<simple_entity::Value> for Value {
	fn from(v: simple_entity::Value) -> Self {
		Value(v)
	}
}

impl WriteGTP for Value {
	fn write_gtp(&self, f: &mut impl io::Write) -> io::Result<()> {
		self.0.write_gtp(f)
	}
}

pub struct Type {
	first: simple_entity::Type,
	second: simple_entity::Type,
}

impl From<(simple_entity::Type, simple_entity::Type)> for Type {
	fn from(pair: (simple_entity::Type, simple_entity::Type)) -> Self {
		let (first, second) = pair;
		Type { first, second }
	}
}

impl Data for Value {
	type Type = Type;

	fn parse<'a, I: Input<'a>>(i: I, t: Self::Type) -> IResult<I, Self> {
		map!(
			i,
			alt!(parse_gtp!(t.first) | parse_gtp!(t.second)),
			From::<simple_entity::Value>::from
		)
	}
}
