use super::Byte;
use super::Input;
use nom::InputIter;
use nom::ParseTo;
use std::str::FromStr;
use std::string::ToString;
//use super::MessagePart;
struct MessagePart {
	msg: Vec<Byte>,
}

// TODO: I'm unhappy with quite a few of the names.

//use self::simple_entity::SimpleEntity;
use nom::IResult;

pub trait Data: Into<MessagePart> {
	type Type;

	// TODO: Which kind of errors do we need to throw?
	fn parse(i: Input, t: Self::Type) -> IResult<Input, Self>;
	fn typed(&self) -> Self::Type;
}

macro_rules! singleton_type {
	( $i: ident ) => {
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
pub mod int;
pub mod float;
pub mod string;
pub mod vertex;
pub mod color;
pub mod motion;
pub mod boolean;

// Compound Entities
pub mod collection;
pub mod list;
pub mod alternatives;
pub mod multiline_list;

// Internal helpers
mod simple_entity;
mod one_liner;
