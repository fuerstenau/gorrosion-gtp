//! (Private)
//!
//! Declarations to work with the types of data defined by GTP.
//! Most of this is publically exported via `gtp_type`,
//! the rest ist internal plumbing.

use super::input::Input;
use super::messages;
use nom::IResult;

// TODO: I'm unhappy with quite a few of the names.

pub trait Typed {
	type Type;
}

// TODO: Add default impl<T> where T::Type: Default
//       when Rust supports this.
pub trait HasType: Typed {
	fn has_type(&self, t: &Self::Type) -> bool;
}

pub trait Data: messages::WriteGTP + Typed + Sized {
	// TODO: Which kind of errors do we need to throw?
	fn parse<'a, I: Input<'a>>(i: I, t: Self::Type) -> IResult<I, Self>;
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

/// Determine the type of an expression
/// in cases where the value and type enums are named in perfect sync.
macro_rules! type_of {
	( $s:expr; $( $t:ident ), * ) => {
		match $s {
			$( Value::$t(_) =>  Type::$t, )*
		}
	}
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
