//! Implement the traits defined by nom
//! so that we can use nom to write our parsers.
//!
//! These are simply wrappers
//! (but not all are simple wrappers)
//! around the interface exposed by the sister modules engine and controller.
//! All GTP specific (pre-)processing happens in these modules.

use super::{Byte, controller, for_t, engine};
use data::int;
use nom::*;
use std::convert::TryFrom;
use std::iter::Enumerate;

/// Implement all the nom interfaces required by input::Input<'a>
/// for a generic type name.
macro_rules! impl_nom {
( $T:ident, $I:ident ) => {
	impl<'a> AtEof for $T<'a> {
		/// While it might be possible in some settings
		/// to determine that the connection has closed
		/// and no further data may arrive,
		/// it is quite irrelevant
		/// considering the particular syntax of GTP.
		/// The only use case would be determining malformed input
		/// which ends without proper termination
		/// but this is currently beyond the scope
		/// of this implementation.
		fn at_eof(&self) -> bool {
			false
		}
	}

	impl<'a> InputLength for $T<'a> {
		fn input_len(&self) -> usize {
			self.bytes().len()
		}
	}

	impl<'a> InputTake for $T<'a> {
		fn take(&self, count: usize) -> Self {
			let bytes = &self.bytes()[0..count];
			$T::from(bytes)
		}

		fn take_split(&self, count: usize) -> (Self, Self) {
			let (prefix, suffix) = self.bytes().split_at(count);
			let prefix = $T::from(prefix);
			let suffix = $T::from(suffix);
			(suffix, prefix)
		}
	}

	impl<'a, R> Slice<R> for $T<'a>
	where
		&'a [Byte]: Slice<R>,
	{
		fn slice(&self, range: R) -> Self {
			let bytes = self.bytes().slice(range);
			$T::from(bytes)
		}
	}

	impl<'a> Offset for $T<'a> {
		fn offset(&self, second: &Self) -> usize {
			self.bytes().offset(second.bytes())
		}
	}

	impl<'a, R> ParseTo<R> for $T<'a>
	where
		&'a [Byte]: ParseTo<R>,
	{
		fn parse_to(&self) -> Option<R> {
			self.bytes().parse_to()
		}
	}

	impl<'a> ParseTo<int::Value> for $T<'a> {
		fn parse_to(&self) -> Option<int::Value> {
			let i: Option<u32> = self.parse_to();
			if let Some(i) = i {
				int::Value::try_from(i).ok()
			} else {
				None
			}
		}
	}

	impl<'a> for_t::Slice for $T<'a> {}
	impl<'a> for_t::ParseTo for $T<'a> {}

	impl<'a, S> Compare<S> for $T<'a>
	where
		&'a [Byte]: Compare<S>,
	{
		fn compare(&self, t: S) -> CompareResult {
			self.bytes().compare(t)
		}

		fn compare_no_case(&self, t: S) -> CompareResult {
			self.bytes().compare_no_case(t)
		}
	}

	impl<'a> InputIter for $T<'a> {
		type Item = Byte;
		type RawItem = Byte;
		type Iter = Enumerate<Self::IterElem>;
		type IterElem = $I<'a>;

		fn iter_indices(&self) -> Self::Iter {
			self.iter_elements().enumerate()
		}

		fn iter_elements(&self) -> Self::IterElem {
			$I::new(self)
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
	impl<'a> UnspecializedInput for $T<'a> {}
}}

type ControllerInput<'a> = controller::Input<'a>;
type ControllerIterator<'a> = controller::Iterator<'a>;
type EngineInput<'a> = engine::Input<'a>;
type EngineIterator<'a> = engine::Iterator<'a>;
impl_nom!(ControllerInput, ControllerIterator);
impl_nom!(EngineInput, EngineIterator);
