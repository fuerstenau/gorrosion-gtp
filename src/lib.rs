#![feature(stmt_expr_attributes)]
// TODO: Disable once all the code lives
#![allow(dead_code)]

#[macro_use]
extern crate nom;

/// Despite its name,
/// GTP feels like a binary protocol
/// and we will treat it as such.
/// Consequently, all “text” is a sequence of bytes.
/// To emphasize that these bytes are not used for their numerical value
/// but rather their property of “most generic kind of data”
/// we will explicitly refer to them as `Byte` instead of `u8`.
type Byte = u8;

mod data;
mod parse;

use parse::Input;

pub mod gtp_type {
	use super::data::*;
	pub type Int = int::Value;
	pub type Float = float::Value;
	pub type String = string::Value;
	pub use self::vertex::Vertex;
	pub use self::color::Color;
	pub type Move = motion::Value;
	pub use self::boolean::Boolean;
	pub type Collection = collection::Value;
	pub type List = list::Value;
	pub type Alternatives = alternatives::Value;
	pub type MultilineList = multiline_list::Value;
}

mod messages;

mod command {
	use super::gtp_type::*;

	// TODO: Macros

	struct Command {
		name: String,
	}
}
