use super::super::messages::WriteGTP;
use super::*;
use std::io;

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum Value {
	False,
	True,
}

impl From<bool> for Value {
	fn from(b: bool) -> Value {
		if b {
			Value::True
		} else {
			Value::False
		}
	}
}

impl From<Value> for bool {
	fn from(v: Value) -> bool {
		match v {
			Value::True => true,
			Value::False => false,
		}
	}
}

impl WriteGTP for Value {
	fn write_gtp(&self, f: &mut impl io::Write) -> io::Result<()> {
		match self {
			Value::False => write!(f, "false"),
			Value::True => write!(f, "true"),
		}
	}
}

singleton_type!(Boolean);

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
			value!(Value::False, tag!("false")) |
			value!(Value::True, tag!("true"))
		)
	}
}
