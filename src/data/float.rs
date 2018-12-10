use super::*;

pub struct Value {
	data: f32,
}

impl From<Value> for MessagePart {
	fn from(Value { data }: Value) -> MessagePart {
		let msg = Vec::from(data.to_string().as_bytes());
		MessagePart { msg }
	}
}

singleton_type!(Float);

impl Typed for Value {
	type Type = Type;
}

impl HasType for Value {
	fn has_type(&self, _t: Self::Type) -> bool {
		true
	}
}

impl Data for Value {
	/// The GTP “specification” does not specify
	/// in which ways a float may be represented.
	/// We therefore simply accept as a float
	/// whatever nom accepts as a float.
	fn parse(i: Input, _t: Self::Type) -> IResult<Input, Self> {
		let result = nom::float(i);
		result.map(|(rem, data)| (rem, Value { data }))
	}
}
