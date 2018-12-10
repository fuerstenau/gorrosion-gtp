use std::str::FromStr;
use super::*;

pub struct Value {
	data: u32,
}

singleton_type!(Int);

impl From<Value> for MessagePart {
	fn from(Value { data }: Value) -> MessagePart {
		let msg = Vec::from(data.to_string().as_bytes());
		MessagePart { msg }
	}
}

impl Data for Value {
	type Type = Type;

	fn parse(i: Input, _t: Self::Type) -> IResult<Input, Self> {
		let digits = nom::digit(i);
		match digits {
			Ok((rem, str)) => {
				let str = str
					.iter_elements()
					.map(nom::AsChar::as_char)
					.collect::<std::string::String>();
				let data = FromStr::from_str(&str).unwrap();
				Ok((rem, Value { data }))
			}
			Err(e) => Err(e),
		}
	}

	fn typed(&self) -> Self::Type {
		Type::Int
	}
}
