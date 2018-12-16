//! (Private)
//!
//! Declarations to work with the types of data defined by GTP.
//! Most of this is publically exported via `gtp_type`,
//! the rest ist internal plumbing.

use super::input::Input;
use super::messages;
use nom::IResult;

// TODO: I'm unhappy with quite a few of the names.

pub trait HasType<T> {
	fn has_type(&self, t: &T) -> bool;
}

pub trait Data: messages::WriteGTP + Sized {
	type Type;

	// TODO: Which kind of errors do we need to throw?
	fn parse<'a, I: Input<'a>>(i: I, t: Self::Type) -> IResult<I, Self>;
}

macro_rules! parse_gtp {
	($i:ident, $t:expr) => {
		Data::parse($i, $t)
	};
}

macro_rules! singleton_type {
	( $i:ident ) => {
		#[derive(PartialEq, Eq)]
		pub enum Type {
			$i,
		}

		impl Default for Type {
			fn default() -> Self {
				Type::$i
			}
		}
	};
}

// Simple Entities
pub mod boolean;
pub mod color;
pub mod float;
pub mod int;
pub mod motion;
pub mod string;
pub mod vertex;

// Compound Entities
pub mod alternatives;
pub mod collection;
pub mod list;
pub mod multiline_list;

// Internal helpers
pub mod one_liner;
pub mod simple_entity;
