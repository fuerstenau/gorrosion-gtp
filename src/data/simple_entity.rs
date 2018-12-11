use super::*;
use nom::IResult;

pub enum Value {
	Int(int::Value),
	Float(float::Value),
	String(string::Value),
	Vertex(vertex::Value),
	Color(color::Value),
	Motion(motion::Value),
	Boolean(boolean::Value),
}

impl From<Value> for MessagePart {
	fn from(val: Value) -> MessagePart {
		match val {
			Value::Int(v) => MessagePart::from(v),
			Value::Float(v) => MessagePart::from(v),
			Value::String(v) => MessagePart::from(v),
			Value::Vertex(v) => MessagePart::from(v),
			Value::Color(v) => MessagePart::from(v),
			Value::Motion(v) => MessagePart::from(v),
			Value::Boolean(v) => MessagePart::from(v),
		}
	}
}

#[derive(PartialEq, Eq)]
pub enum Type {
	Int,
	Float,
	String,
	Vertex,
	Color,
	Motion,
	Boolean,
}

impl Typed for Value {
	type Type = Type;
}

impl HasType for Value {
	fn has_type(&self, t: &Self::Type) -> bool {
		#[rustfmt::skip]
		t == &type_of!(self;
			Int,
			Float,
			String,
			Vertex,
			Color,
			Motion,
			Boolean
		)
	}
}

macro_rules! parse {
	( $i:expr, $e:expr; $( ($t:ident, $m:ident) ), * ) => {
		match $e {
			$( Type::$t =>
				$m::Value::parse($i, $m::Type::default())
					.map(|(rm, rs)| (rm, Value::$t(rs))),
			)*
		}
	}
}

impl Data for Value {
	fn parse(i: Input, t: Self::Type) -> IResult<Input, Self> {
		#[rustfmt::skip]
		parse!(i, t;
			(Int, int),
			(Float, float),
			(String, string),
			(Vertex, vertex),
			(Color, color),
			(Motion, motion),
			(Boolean, boolean)
		)
	}
}
