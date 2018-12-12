use super::Byte;
use std::convert::From;

mod nom;

use nom::*;

macro_rules! for_some {
	( $mod:ident $trait:ident for $( $params:tt ); * ) => {
		pub trait $trait
		where
			$(Self: $mod::$trait<$params> ), *
		{}
	}
}

mod for_t {
	for_some!(nom ParseTo for f32; u32; u8);

	use std::ops::{Range, RangeFrom, RangeTo};
	type uRange = Range<usize>;
	type uRangeTo = RangeTo<usize>;
	type uRangeFrom = RangeFrom<usize>;
	for_some!(nom Slice for uRange; uRangeTo; uRangeFrom);
}

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
	Self: Compare<&'a [Byte; 1]>,
	Self: InputIter<Item = Byte, RawItem = Byte>,
	Self: UnspecializedInput,
	Self: for_t::ParseTo,
	Self: for_t::Slice,
{
	//	type Iter: Iterator;

	fn bytes(&self) -> &'a [Byte];
}
