use super::Byte;
use std::iter;

const DISCARD: [Byte; 31] = [
	0, 1, 2, 3, 4, 5, 6, 7, 8, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
	22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 127,
]; // “Control characters”: 0 – 8, 11 – 31, 127
const SPACE: [Byte; 2] = [9, 32]; // " \t"
const NEWLINE: Byte = 10; // "\n"
const COMMENT: Byte = 35; // "#"

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

	fn discard(&self, pos: usize) -> bool {
		DISCARD.contains(&self.bytes[pos])
	}
}

impl<'a> iter::Iterator for Iterator<'a> {
	type Item = Byte;

	fn next(&mut self) -> Option<Self::Item> {
		if self.next >= self.bytes.len() {
			None
		} else if self.discard(self.next) {
			self.next += 1;
			self.next()
		} else {
			let res = self.bytes[self.next];
			self.next += 1;
			Some(res)
		}
	}
}
