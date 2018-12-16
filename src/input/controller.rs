use super::Byte;
use super::{coerce_whitespace, discard};
use std::iter;

#[derive(Clone)]
pub struct Input<'a> {
	bytes: &'a [Byte],
}

impl<'a> Input<'a> {
	#[doc(hidden)]
	// TODO: This can probably made private with little effort
	//       since most usages of this field
	//       need to be moved in here anyway.
	pub fn bytes(&self) -> &'a [Byte] {
		self.bytes
	}
}

impl<'a> From<&'a [Byte]> for Input<'a> {
	fn from(bytes: &'a [Byte]) -> Self {
		Input { bytes }
	}
}

impl<'a> super::Input<'a> for Input<'a> {}

pub struct Iterator<'a> {
	bytes: &'a [Byte],
	/// One more than the position of the last element that was output.
	/// If we are not at the end of the iteration
	/// and there are no discardable bytes,
	/// it happens to be the position of the next element.
	next: usize,
}

impl<'a> Iterator<'a> {
	pub fn new(i: &Input<'a>) -> Self {
		let bytes = i.bytes;
		let next = 0;
		Iterator { bytes, next }
	}
}

impl<'a> iter::Iterator for Iterator<'a> {
	type Item = Byte;

	fn next(&mut self) -> Option<Self::Item> {
		macro_rules! next_byte {
			() => {
				self.bytes[self.next]
			};
		}
		if self.next >= self.bytes.len() {
			None
		} else if discard(next_byte!()) {
			self.next += 1;
			self.next()
		} else {
			let res = next_byte!();
			self.next += 1;
			Some(coerce_whitespace(res))
		}
	}
}

pub struct Enumerator<'a>(Iterator<'a>);

impl<'a> Enumerator<'a> {
	pub fn new(i: &Input<'a>) -> Self {
		Enumerator(Iterator::new(i))
	}
}

impl<'a> iter::Iterator for Enumerator<'a> {
	type Item = (usize, Byte);

	fn next(&mut self) -> Option<Self::Item> {
		let Enumerator(iter) = self;
		let byte = iter.next()?;
		Some((iter.next - 1, byte))
	}
}
