//! Implement the traits defined by nom
//! so that we can use nom to write our parsers.
//! Most of these are simple wrappers,
//! the GTP specific (pre-)processing happens in the submodule `iter`.

use nom::*;
use super::{Input, Byte};
use ::data::int;
use std::iter::Enumerate;
use std::convert::TryFrom;

mod iter;

use self::iter::InputIterator;

impl<'a> AtEof for Input<'a> {
	/// While it might be possible in some settings
	/// to determine that the connection has closed
	/// and no further data may arrive,
	/// it is quite irrelevant considering the particular syntax of GTP.
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

impl<'a, R> Slice<R> for Input<'a>
where
	&'a [Byte]: Slice<R>,
{
	fn slice(&self, range: R) -> Self {
		let bytes = self.bytes.slice(range);
		Input { bytes }
	}
}

impl<'a> Offset for Input<'a> {
	fn offset(&self, second: &Self) -> usize {
		self.bytes.offset(second.bytes)
	}
}

impl<'a, R> ParseTo<R> for Input<'a>
where
	&'a [Byte]: ParseTo<R>,
{
	fn parse_to(&self) -> Option<R> {
		self.bytes.parse_to()
	}
}

impl<'a> ParseTo<int::Value> for Input<'a> {
	fn parse_to(&self) -> Option<int::Value> {
		let i: Option<u32> = self.parse_to();
		if let Some(i) = i {
			int::Value::try_from(i).ok()
		} else {
			None
		}
	}
}

impl<'a, T> Compare<T> for Input<'a>
where
	&'a [Byte]: Compare<T>,
{
	fn compare(&self, t: T) -> CompareResult {
		self.bytes.compare(t)
	}

	fn compare_no_case(&self, t: T) -> CompareResult {
		self.bytes.compare_no_case(t)
	}
}

impl<'a> InputIter for Input<'a> {
	type Item = Byte;
	type RawItem = Byte;
	type Iter = Enumerate<Self::IterElem>;
	type IterElem = iter::InputIterator<'a>;

	fn iter_indices(&self) -> Self::Iter {
		self.iter_elements().enumerate()
	}

	fn iter_elements(&self) -> Self::IterElem {
		InputIterator::new(self)
	}

	fn position<P>(&self, predicate: P) -> Option<usize>
	where
		P: Fn(Self::RawItem) -> bool,
	{
		let mut iter = self.iter_elements();
		loop {
			if let Some(elem) = iter.next() {
				if predicate(elem) {
					continue;
				} else {
					break Some(iter.last_pos());
				}
			} else {
				break None;
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

/// This allows us to use a default implementation for InputTakeAtPosition.
impl<'a> UnspecializedInput for Input<'a> {}
