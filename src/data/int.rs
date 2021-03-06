use super::super::messages::WriteGTP;
use super::*;
use std::convert::TryFrom;
use std::io;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Value {
	data: u32,
}

#[derive(Debug)]
pub struct TryFromIntError(());

impl TryFrom<u32> for Value {
	type Error = TryFromIntError;

	fn try_from(data: u32) -> Result<Self, Self::Error> {
		if data < (1 << 31) {
			Ok(Value { data })
		} else {
			Err(TryFromIntError(()))
		}
	}
}

impl From<Value> for u32 {
	fn from(v: Value) -> u32 {
		v.data
	}
}

impl From<Value> for i32 {
	fn from(v: Value) -> i32 {
		// This should be safe to unwrap
		// as we should not instantiate
		// a Value with data >= 2^31
		// in the first place.
		i32::try_from(v.data).unwrap()
	}
}

impl WriteGTP for Value {
	fn write_gtp(&self, f: &mut impl io::Write) -> io::Result<()> {
		write!(f, "{}", self.data)
	}
}

singleton_type!(Int);

impl HasType<Type> for Value {
	fn has_type(&self, _t: &Type) -> bool {
		true
	}
}

impl Data for Value {
	type Type = Type;

	fn parse<'a, I: Input<'a>>(i: I, _t: &Self::Type) -> IResult<I, Self> {
		flat_map!(i, nom::digit, parse_to!(Self))
	}
}
