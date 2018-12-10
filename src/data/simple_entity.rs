use super::*;
use nom::IResult;
use std::convert::TryFrom;

pub enum Type {
	Int,
	Float,
	String,
	Vertex,
	Color,
	Motion,
	Boolean,
}

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
	type Type = Type;

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

	fn typed(&self) -> Self::Type {
		#[rustfmt::skip]
		type_of!(self;
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

pub trait SimpleEntity: Data + TryFrom<Value> + Into<Value> {}
