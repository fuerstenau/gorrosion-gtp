use super::*;

pub struct Value {
	color: color::Value,
	vertex: vertex::Value,
}

impl Value {
	pub fn color(&self) -> &color::Value {
		&self.color
	}

	pub fn vertex(&self) -> &vertex::Value {
		&self.vertex
	}
}

singleton_type!(Motion);

impl Typed for Value {
	type Type = Type;
}

impl HasType for Value {
	fn has_type(&self, _t: &Self::Type) -> bool {
		true
	}
}

impl Data for Value {
	fn parse<'a, I: Input<'a>>(i: I, _t: Self::Type) -> IResult<I, Self> {
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
}
