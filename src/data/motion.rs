use super::super::messages::WriteGTP;
use super::*;
use std::io;

#[derive(Clone, Debug, PartialEq, Eq)]
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

impl WriteGTP for Value {
	fn write_gtp(&self, f: &mut impl io::Write) -> io::Result<()> {
		self.color.write_gtp(f)?;
		write!(f, " ")?;
		self.vertex.write_gtp(f)
	}
}

singleton_type!(Motion);

impl HasType<Type> for Value {
	fn has_type(&self, _t: &Type) -> bool {
		true
	}
}

impl Data for Value {
	type Type = Type;

	fn parse<'a, I: Input<'a>>(i: I, _t: &Self::Type) -> IResult<I, Self> {
		let color_t = &color::Type::default();
		let vertex_t = &vertex::Type::default();
		#[rustfmt::skip]
		do_parse!(i,
			color: parse_gtp!(color_t) >>
			tag!(" ") >>
			vertex: parse_gtp!(vertex_t) >>
			(Value { color, vertex })
		)
	}
}
