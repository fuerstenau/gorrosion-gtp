use super::Byte;
use nom::*;
use std::iter;

const DISCARD: [Byte; 31] = [
	0, 1, 2, 3, 4, 5, 6, 7, 8, 11, 12, 13, 14, 15, 16, 17, 18, 19,
	20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 127,
]; // “Control characters”: 0 – 8, 11 – 31, 127
const SPACE: [Byte; 2] = [9, 32]; // " \t"
const NEWLINE: Byte = 10; // "\n"
const COMMENT: Byte = 35; // "#"

pub struct Input<'a> {
	bytes: &'a [Byte],
}

impl<'a> std::convert::From<&'a [u8]> for Input<'a> {
	fn from(bytes: &'a [u8]) -> Self {
		Input { bytes }
	}
}

impl<'a> AtEof for Input<'a> {
	/// While it might be possible in some settings
	/// to determine that the connection has closed
	/// and no further data may arrive
	/// it is quite considering the particular syntax of GTP.
	/// The only use case would be determining malformed input
	/// which ends without proper termination
	/// but this is currently beyond the scope
	/// of this implementation.
	fn at_eof(&self) -> bool {
		false
	}
}

impl<'a> InputLength for Input<'a> {
	fn input_len(&self) -> usize {
		self.bytes.len()
	}
}

impl<'a> InputTake for Input<'a> {
	fn take(&self, count: usize) -> Self {
		let bytes = &self.bytes[0..count];
		Input { bytes }
	}

	fn take_split(&self, count: usize) -> (Self, Self) {
		let (prefix, suffix) = self.bytes.split_at(count);
		let prefix = Input::from(prefix);
		let suffix = Input::from(suffix);
		(suffix, prefix)
	}
}

pub struct InputIterator<'a> {
	bytes: &'a [Byte],
	/// One more than the position of the last element
	/// that was output.
	/// If we are not at the end of the iteration
	/// and there are no discardable bytes,
	/// it happens to be the position of the next element.
	next: usize,
}

impl<'a> InputIterator<'a> {
	fn last_pos(&self) -> usize {
		self.next - 1
	}

	fn discard(&self, pos: usize) -> bool {
		DISCARD.contains(&self.bytes[pos])
	}
}

impl<'a> iter::Iterator for InputIterator<'a> {
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

impl<'a> InputIter for Input<'a> {
	type Item = Byte;
	type RawItem = Byte;
	type Iter = iter::Enumerate<Self::IterElem>;
	type IterElem = InputIterator<'a>;

	fn iter_indices(&self) -> Self::Iter {
		self.iter_elements().enumerate()
	}

	fn iter_elements(&self) -> Self::IterElem {
		let bytes = self.bytes;
		let next = 0;
		InputIterator { bytes, next }
	}

	fn position<P>(&self, predicate: P) -> Option<usize>
	where
		P: Fn(Self::RawItem) -> bool
	{
		let mut iter = self.iter_elements();
		loop {
			if let Some(elem) = iter.next() {
				if predicate(elem) {
					continue
				} else {
					break Some(iter.last_pos())
				}
			} else {
				break None
			}
		}
	}

	fn slice_index(&self, count: usize) -> Option<usize> {
		let mut iter = self.iter_elements();
		let nth = iter.nth(count);
		if nth.is_none() {
			None
		} else {
			Some(iter.last_pos())
		}
	}
}