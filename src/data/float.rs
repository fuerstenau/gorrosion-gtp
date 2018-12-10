use super::*;

pub struct Value {
	data: f32,
}

singleton_type!(Float);

impl From<Value> for MessagePart {
	fn from(Value { data }: Value) -> MessagePart {
		let msg = Vec::from(data.to_string().as_bytes());
		MessagePart { msg }
	}
}

impl Data for Value {
	type Type = Type;

	/// The GTP “specification” does not specify
	/// in which ways a float may be represented.
	/// We therefore simply accept as a float
	/// whatever nom accepts as a float.
	fn parse(i: Input, _t: Self::Type) -> IResult<Input, Self> {
		let result = nom::float(i);
		match result {
			Ok((rem, data)) => Ok((rem, Value { data })),
			Err(e) => Err(e),
		}
	}

	fn typed(&self) -> Self::Type {
		Type::default()
	}
}
