use super::*;

pub struct Value {
	color: color::Value,
	vertex: vertex::Value,
}

singleton_type!(Motion);

impl From<Value> for MessagePart {
	fn from(m: Value) -> MessagePart {
		let mut msg = MessagePart::from(m.color).msg;
		msg.extend(b" ");
		msg.append(&mut MessagePart::from(m.vertex).msg);
		MessagePart { msg }
	}
}

impl Data for Value {
	type Type = Type;

	fn parse(i: Input, _t: Self::Type) -> IResult<Input, Self> {
		let parse_color =
			|i| color::Value::parse(i, color::Type::default());
		let parse_vertex =
			|i| vertex::Value::parse(i, vertex::Type::default());
		#[rustfmt::skip]
		do_parse!(i,
			color: call!(parse_color) >>
			tag!(" ") >>
			vertex: call!(parse_vertex) >>
			(Value { color, vertex })
		)
	}

	fn typed(&self) -> Self::Type {
		Type::default()
	}
}
