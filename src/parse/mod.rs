use super::Byte;
use std::convert::From;

mod nom;
mod controller;

use nom::*;

mod for_t {
	macro_rules! for_some {
		( $mod:ident $trait:ident for $( $params:tt ); * ) => {
			pub trait $trait
			where
				$(Self: $mod::$trait<$params> ), *
			{}
		}
	}

	for_some!(nom ParseTo for f32; u32; u8);

	use std::ops::{Range, RangeFrom, RangeTo};
	type RangeUsize = Range<usize>;
	type RangeToUsize = RangeTo<usize>;
	type RangeFromUsize = RangeFrom<usize>;
	for_some!(nom Slice for RangeUsize; RangeToUsize; RangeFromUsize);
}

/// This is a simple collection trait,
/// comprising all the traits that are currently used
/// to parse the inputs.
/// Only engine::Input and controller::Input should implement this trait.
pub trait Input<'a>
where
	Self: From<&'a [u8]>,
	Self: Clone,
	Self: AtEof,
	Self: InputLength,
	Self: InputTake,
	Self: Offset,
	Self: Compare<&'a [Byte]>,
	Self: Compare<&'a str>,
	Self: InputIter<Item = Byte, RawItem = Byte>,
	Self: UnspecializedInput,
	Self: for_t::ParseTo,
	Self: for_t::Slice,
{}
