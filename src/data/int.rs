use nom::ParseTo;
use super::*;
use std::convert::TryFrom;

pub struct Value {
	data: u32,
}

pub struct TryFromIntError(());

impl TryFrom<u32> for Value
{
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

singleton_type!(Int);

impl Typed for Value {
	type Type = Type;
}

impl HasType for Value {
	fn has_type(&self, _t: Self::Type) -> bool {
		true
	}
}

impl Data for Value {
	// FIXME: Ensure data < 2^31
	fn parse(i: Input, _t: Self::Type) -> IResult<Input, Self> {
		let digits = nom::digit(i);
		match digits {
			Ok((rem, str)) => {
				let data = str.parse_to().unwrap();
				Ok((rem, Value { data }))
			}
			Err(e) => Err(e),
		}
	}
}
