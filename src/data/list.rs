use super::*;
use std::io;
use super::super::messages::WriteGTP;

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

pub type Type = collection::Type;
