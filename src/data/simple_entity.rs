use super::*;
use nom::IResult;
use std::io;
use super::super::messages::Writable;

pub enum Value {
	Int(int::Value),
	Float(float::Value),
	String(string::Value),
	Vertex(vertex::Value),
	Color(color::Value),
	Motion(motion::Value),
	Boolean(boolean::Value),
}

impl Writable for Value {
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
	fn parse<'a, I: Input<'a>>(i: I, t: Self::Type) -> IResult<I, Self> {
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
