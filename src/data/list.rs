use super::super::messages::WriteGTP;
use super::*;
use std::io;

pub struct Value {
	t: collection::Type,
	data: Vec<collection::Value>,
}

impl Value {
	pub fn is_empty(&self) -> bool {
		let data = &self.data;
		data.is_empty() || (data.len() == 1 && data[0].is_empty())
	}
}

impl WriteGTP for Value {
	fn write_gtp(&self, f: &mut impl io::Write) -> io::Result<()> {
		if self.data.is_empty() {
			Ok(())
		} else {
			let mut iter = self.data.iter();
			// The call to `unwrap()` should be safe
			// due to the conditional branch we are in.
			iter.next().unwrap().write_gtp(f)?;
			for e in iter {
				write!(f, " ")?;
				e.write_gtp(f)?;
			}
			Ok(())
		}
	}
}

#[derive(Clone)]
pub struct Type(collection::Type);

impl From<collection::Type> for Type {
	fn from(t: collection::Type) -> Self {
		Type(t)
	}
}

impl From<simple_entity::Type> for Type {
	fn from(t: simple_entity::Type) -> Self {
		Type(From::from(t))
	}
}

impl Data for Value {
	type Type = Type;

	fn parse<'a, I: Input<'a>>(i: I, t: &Self::Type) -> IResult<I, Self> {
		let Type(t) = t.clone();
		#[rustfmt::skip]
		do_parse!(i,
			data: separated_list!(tag!(" "), parse_gtp!(&t)) >>
			(Value { t, data })
		)
	}
}
