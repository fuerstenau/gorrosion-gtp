use nom::ParseTo;
use super::*;

pub struct Value {
	data: u32,
}

impl From<u32> for Value {
	fn from(data: u32) -> Value {
		Value { data }
	}
}

impl From<Value> for u32 {
	fn from(v: Value) -> u32 {
		v.data
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
