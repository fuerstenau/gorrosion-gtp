use super::Byte;
use std::convert::From;

mod nom;

#[derive(Clone, Copy, Debug)]
pub struct Input<'a> {
	bytes: &'a [Byte],
}

impl<'a> From<&'a [u8]> for Input<'a> {
	fn from(bytes: &'a [u8]) -> Self {
		Input { bytes }
	}
}

