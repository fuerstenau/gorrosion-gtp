use super::Byte;
use super::{discard, coerce_whitespace, starts_comment, newline};
use std::iter;


#[derive(Clone)]
pub struct Input<'a> {
	bytes: &'a [Byte],
	start_of_line: bool,
}

impl<'a> Input<'a> {
	#[doc(hidden)]
	pub fn bytes(&self) -> &'a [Byte] {
		self.bytes
	}
}

impl<'a> From<&'a [Byte]> for Input<'a> {
	fn from(bytes: &'a [Byte]) -> Self {
		// TODO: Is this the correct behaviour?
		let start_of_line = true;
		Input { bytes, start_of_line }
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
	start_of_line: bool,
}

impl<'a> Iterator<'a> {
	pub fn new(i: &Input<'a>) -> Self {
		let bytes = i.bytes;
		let next = 0;
		let start_of_line = i.start_of_line;
		Iterator { bytes, next, start_of_line }
	}

	fn skip_comment(&mut self) -> Option<Byte> {
		let len = self.bytes.len();
		macro_rules! next_byte {() => {self.bytes[self.next]}}
		loop {
			self.next += 1;
			if self.next >= len {
				break None;
			} else if newline(&next_byte!()) {
				break Some(next_byte!());
			} else {
				continue;
			}
		}
	}
}

impl<'a> iter::Iterator for Iterator<'a> {
	type Item = Byte;

	// TODO: Skip empty lines
	fn next(&mut self) -> Option<Self::Item> {
		macro_rules! next_byte {() => {self.bytes[self.next]}}
		let len = self.bytes.len();
		let res = if self.next >= len {
			return None
		} else if discard(&next_byte!()) {
			self.next += 1;
			self.next()?
		} else if starts_comment(&next_byte!()) {
			self.skip_comment()?
		} else {
			next_byte!()
		};
		if newline(&res) {
			self.start_of_line = true;
		}
		self.next += 1;
		Some(coerce_whitespace(res))
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
