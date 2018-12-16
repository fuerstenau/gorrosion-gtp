use super::super::messages::WriteGTP;
use super::*;
use std::io;

type Byte = u8;

pub struct Value {
	data: Vec<Byte>,
}

impl From<Value> for Vec<Byte> {
	fn from(v: Value) -> Vec<Byte> {
		v.data
	}
}

impl WriteGTP for Value {
	fn write_gtp(&self, f: &mut impl io::Write) -> io::Result<()> {
		f.write_all(&self.data)
	}
}

singleton_type!(String);

impl HasType<Type> for Value {
	fn has_type(&self, _t: &Type) -> bool {
		true
	}
}

impl Data for Value {
	type Type = Type;

	fn parse<'a, I: Input<'a>>(i: I, _t: Self::Type) -> IResult<I, Self> {
		let result = take_until_either!(i, b" \n");
		match result {
			Ok((rem, data)) => {
				let data = data.iter_elements().collect();
				Ok((rem, Value { data }))
			}
			Err(e) => Err(e),
		}
	}
}
