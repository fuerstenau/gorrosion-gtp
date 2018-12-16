use super::super::messages::WriteGTP;
use super::*;
use std::io;

pub struct Value {
	t: one_liner::Type,
	data: Vec<one_liner::Value>,
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

#[derive(Clone, Debug)]
pub struct Type(one_liner::Type);

impl From<one_liner::Type> for Type {
	fn from(t: one_liner::Type) -> Self {
		Type(t)
	}
}

impl From<simple_entity::Type> for Type {
	fn from(t: simple_entity::Type) -> Self {
		Type(t.into())
	}
}

impl From<list::Type> for Type {
	fn from(t: list::Type) -> Self {
		Type(t.into())
	}
}

impl From<collection::Type> for Type {
	fn from(t: collection::Type) -> Self {
		Type(t.into())
	}
}

impl From<alternatives::Type> for Type {
	fn from(t: alternatives::Type) -> Self {
		Type(t.into())
	}
}

impl Data for Value {
	type Type = Type;

	fn parse<'a, I: Input<'a>>(i: I, t: &Self::Type) -> IResult<I, Self> {
		let Type(t) = t.clone();
		do_parse!(i,
			data: separated_list!(tag!("\n"), parse_gtp!(&t)) >>
			(Value { t, data })
		)
	}
}
