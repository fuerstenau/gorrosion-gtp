use super::*;

pub struct Value {
	data: Vec<Byte>,
}

singleton_type!(String);

impl From<Value> for MessagePart {
	fn from(Value { data }: Value) -> MessagePart {
		let msg = data;
		MessagePart { msg }
	}
}

impl Data for Value {
	type Type = Type;

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

	fn typed(&self) -> Self::Type {
		Type::default()
	}
}
