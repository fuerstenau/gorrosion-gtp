use super::super::messages::WriteGTP;
use super::*;
use std::io;

pub struct Value {
	data: f32,
}

impl From<f32> for Value {
	fn from(data: f32) -> Value {
		Value { data }
	}
}

impl From<Value> for f32 {
	fn from(v: Value) -> f32 {
		v.data
	}
}

impl WriteGTP for Value {
	fn write_gtp(&self, f: &mut impl io::Write) -> io::Result<()> {
		write!(f, "{}", self.data)
	}
}

singleton_type!(Float);

impl HasType<Type> for Value {
	fn has_type(&self, _t: &Type) -> bool {
		true
	}
}

impl Data for Value {
	type Type = Type;

	/// The GTP “specification” does not specify
	/// in which ways a float may be represented.
	/// We therefore simply accept as a float
	/// whatever nom accepts as a float.
	fn parse<'a, I: Input<'a>>(i: I, _t: &Self::Type) -> IResult<I, Self> {
		let result = nom::float(i);
		result.map(|(rem, data)| (rem, Value { data }))
	}
}
