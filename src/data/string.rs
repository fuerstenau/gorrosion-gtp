use super::*;
use nom::InputIter;

pub struct Value {
	data: Vec<Byte>,
}

impl From<Value> for Vec<Byte> {
	fn from(v: Value) -> Vec<Byte> {
		v.data
	}
}

singleton_type!(String);

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
		let result = take_until_either!(i, b" \n");
		match result {
			Ok((rem, data)) => {
				let data = data.iter_elements().collect();
				Ok((rem, Value { data }))
			}
			Err(e) => Err(e),
		}
	}
}
