//! Implement the traits defined by nom
//! so that we can use nom to write our parsers.
//!
//! These are simply wrappers
//! (but not all are simple wrappers)
//! around the interface exposed by the sister modules engine and controller.
//! All GTP specific (pre-)processing happens in these modules.

use super::{controller, engine, for_t, Byte};
use data::int;
use nom::*;
use std::convert::TryFrom;

/// Implement all the nom interfaces required by input::Input<'a>
/// for a generic type name.
macro_rules! impl_nom {
	( $T:ident, $I:ident, $E:ident ) => {
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
			// TODO: Is this the correct behaviour?
			//       The rest of the nom interface suggests
			//       that by “length of the input“
			//       the bytewise length is meant
			//       instead of the number of elements
			//       returned by the iterator
			//       but it is not made explicit in the documentation.
			fn input_len(&self) -> usize {
				self.bytes().len()
			}
		}

		impl<'a> InputTake for $T<'a> {
			fn take(&self, count: usize) -> Self {
				let bytes = &self.bytes()[0..count];
				$T::from(bytes)
			}

			// FIXME: This behaviour is incorrect for engine::Input,
			//        as it fails to respect comments and empty lines.
			fn take_split(&self, count: usize) -> (Self, Self) {
				let (prefix, suffix) =
					self.bytes().split_at(count);
				let prefix = $T::from(prefix);
				let suffix = $T::from(suffix);
				(suffix, prefix)
			}
		}

		impl<'a, R> Slice<R> for $T<'a>
		where
			&'a [Byte]: Slice<R>,
		{
			// TODO: Is this the correct behaviour?
			//       The rest of the nom interface suggests
			//       that by “length of the input“
			//       the bytewise length is meant
			//       instead of the number of elements
			//       returned by the iterator
			//       but it is not made explicit in the documentation.
			// FIXME: This behaviour is incorrect for engine::Input,
			//        as it fails to respect comments and empty lines.
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
			for<'b> &'b [Byte]: ParseTo<R>,
		{
			fn parse_to(&self) -> Option<R> {
				let str: Vec<Byte> =
					self.iter_elements().collect();
				str.as_bytes().parse_to()
			}
		}

		impl<'a> ParseTo<int::Value> for $T<'a> {
			fn parse_to(&self) -> Option<int::Value> {
				let i: u32 = self.parse_to()?;
				int::Value::try_from(i).ok()
			}
		}

		impl<'a> for_t::Slice for $T<'a> {}
		impl<'a> for_t::ParseTo for $T<'a> {}

		impl<'a, S> Compare<S> for $T<'a>
		where
			&'a [Byte]: Compare<S>,
		{
			// FIXME: Needs to iterate over iter_elements.
			fn compare(&self, t: S) -> CompareResult {
				self.bytes().compare(t)
			}

			// FIXME: Needs to iterate over iter_elements.
			fn compare_no_case(&self, t: S) -> CompareResult {
				self.bytes().compare_no_case(t)
			}
		}

		impl<'a> InputIter for $T<'a> {
			type Item = Byte;
			type RawItem = Byte;
			type Iter = $E<'a>;
			type IterElem = $I<'a>;

			fn iter_indices(&self) -> Self::Iter {
				$E::new(self)
			}

			fn iter_elements(&self) -> Self::IterElem {
				$I::new(self)
			}

			fn position<P>(&self, predicate: P) -> Option<usize>
			where
				P: Fn(Self::RawItem) -> bool,
			{
				let mut iter = self.iter_indices();
				loop {
					if let Some((index, elem)) = iter.next()
					{
						if predicate(elem) {
							continue;
						} else {
							break Some(index);
						}
					} else {
						break None;
					}
				}
			}

			fn slice_index(&self, count: usize) -> Option<usize> {
				let mut iter = self.iter_indices();
				let (res, _) = iter.nth(count)?;
				Some(res)
			}
		}

		/// This allows us to use a default implementation
		/// for `InputTakeAtPosition`.
		impl<'a> UnspecializedInput for $T<'a> {}
	};
}

type ControllerInput<'a> = controller::Input<'a>;
type ControllerIterator<'a> = controller::Iterator<'a>;
type ControllerEnumerator<'a> = controller::Enumerator<'a>;
type EngineInput<'a> = engine::Input<'a>;
type EngineIterator<'a> = engine::Iterator<'a>;
type EngineEnumerator<'a> = engine::Enumerator<'a>;
impl_nom!(ControllerInput, ControllerIterator, ControllerEnumerator);
impl_nom!(EngineInput, EngineIterator, EngineEnumerator);
