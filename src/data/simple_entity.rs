use super::super::messages::WriteGTP;
use super::*;
use nom::IResult;
use std::io;

pub enum Value {
	Int(int::Value),
	Float(float::Value),
	String(string::Value),
	Vertex(vertex::Value),
	Color(color::Value),
	Motion(motion::Value),
	Boolean(boolean::Value),
}

macro_rules! impl_froms {
	( $(($t:ident, $m:ident)), * ) => {
		$(impl From<$m::Value> for Value {
			fn from(v: $m::Value) -> Self {
				Value::$t(v)
			}
		})*
	}
}

impl_froms!(
	(Int, int),
	(Float, float),
	(String, string),
	(Vertex, vertex),
	(Color, color),
	(Motion, motion),
	(Boolean, boolean)
);

impl WriteGTP for Value {
	fn write_gtp(&self, f: &mut impl io::Write) -> io::Result<()> {
		match self {
			Value::Int(v) => v.write_gtp(f),
			Value::Float(v) => v.write_gtp(f),
			Value::String(v) => v.write_gtp(f),
			Value::Vertex(v) => v.write_gtp(f),
			Value::Color(v) => v.write_gtp(f),
			Value::Motion(v) => v.write_gtp(f),
			Value::Boolean(v) => v.write_gtp(f),
		}
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
	Int,
	Float,
	String,
	Vertex,
	Color,
	Motion,
	Boolean,
}

impl HasType<Type> for Value {
	fn has_type(&self, t: &Type) -> bool {
		match (self, t) {
			(Value::Int(_), Type::Int) => true,
			(Value::Float(_), Type::Float) => true,
			(Value::String(_), Type::String) => true,
			(Value::Vertex(_), Type::Vertex) => true,
			(Value::Color(_), Type::Color) => true,
			(Value::Motion(_), Type::Motion) => true,
			(Value::Boolean(_), Type::Boolean) => true,
			_ => false,
		}
	}
}

macro_rules! parse {
	( $in:expr, $e:expr; $( ($t:ident, $m:ident) ), * ) => {
		match $e {
			$( Type::$t => {
				let t = &$m::Type::default();
				// TODO: map!($in, parse_gtp!(t), From::from)
				let result = $m::Value::parse($in, t);
				result.map(|(i, v)| (i, From::from(v)))
			} )*
		}
	}
}

impl Data for Value {
	type Type = Type;

	fn parse<'a, I: Input<'a>>(i: I, t: &Self::Type) -> IResult<I, Self> {
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
