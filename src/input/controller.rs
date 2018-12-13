use super::Byte;
use super::{discard, coerce_whitespace};
use std::iter;


#[derive(Clone)]
pub struct Input<'a> {
	bytes: &'a [Byte],
}

impl<'a> Input<'a> {
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

// FIXME: Convert tab to space
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

	pub fn last_pos(&self) -> usize {
		self.next - 1
	}
}

impl<'a> iter::Iterator for Iterator<'a> {
	type Item = Byte;

	fn next(&mut self) -> Option<Self::Item> {
		if self.next >= self.bytes.len() {
			None
		} else if discard(&self.bytes[self.next]) {
			self.next += 1;
			self.next()
		} else {
			let res = self.bytes[self.next];
			self.next += 1;
			Some(coerce_whitespace(res))
		}
	}
}
