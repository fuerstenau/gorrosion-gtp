use super::super::messages::WriteGTP;
use super::*;
use std::io;

pub struct Value {
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

pub type Type = one_liner::Type;

impl Data for Value {
	type Type = Type;

	fn parse<'a, I: Input<'a>>(i: I, t: Self::Type) -> IResult<I, Self> {
		unimplemented!()
	}
}
