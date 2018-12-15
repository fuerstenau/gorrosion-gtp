//! (Private)
//!
//! Tooling to deal with the fact that we get bytes but want data.
//! General (pre-)processing happens here,
//! so the stuff in `data` can focus on the interesting parts of parsing.

use super::Byte;
use std::convert::From;
use nom::*;

mod nom;
pub mod controller;
pub mod engine;

const DISCARD: [Byte; 31] = [
	0, 1, 2, 3, 4, 5, 6, 7, 8, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
	22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 127,
]; // “Control characters”: 0 – 8, 11 – 31, 127
const WHITESPACE: [Byte; 2] = [9, 32]; // " \t"

const SPACE: Byte = 9; // " "
const NEWLINE: Byte = 10; // "\n"
const COMMENT: Byte = 35; // "#"

fn discard(b: &Byte) -> bool {
	DISCARD.contains(b)
}

fn whitespace(b: &Byte) -> bool {
	WHITESPACE.contains(b)
}

fn newline(b: &Byte) -> bool {
	*b == NEWLINE
}

fn starts_comment(b: &Byte) -> bool {
	*b == COMMENT
}

fn coerce_whitespace(b: Byte) -> Byte {
		if whitespace(&b) {
			SPACE
		} else {
			b
		}
}

/// This is a simple collection trait,
/// comprising all the traits that are currently used to parse the inputs.
/// Only engine::Input and controller::Input should implement this trait.
#[doc(hidden)]
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

mod for_t {
	macro_rules! for_some {
		( $mod:ident $trait:ident for $( $params:tt ); * ) => {
			pub trait $trait
			where
				$(Self: $mod::$trait<$params> ), *
			{}
		}
	}

	use super::super::gtp_type::Int;
	for_some!(nom ParseTo for f32; u32; u8; Int);

	use std::ops::{Range, RangeFrom, RangeTo};
	type RangeUsize = Range<usize>;
	type RangeToUsize = RangeTo<usize>;
	type RangeFromUsize = RangeFrom<usize>;
	for_some!(nom Slice for RangeUsize; RangeToUsize; RangeFromUsize);
}
