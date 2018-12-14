#![feature(stmt_expr_attributes)]
#![feature(try_from)]
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
mod input;

pub mod gtp_type;
pub mod messages;

mod command {
	use super::gtp_type::*;

	// TODO: Macros

	struct Command {
		name: String,
	}
}
