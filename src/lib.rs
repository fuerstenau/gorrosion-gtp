#![feature(try_from)]
#![feature(stmt_expr_attributes)]

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

use messages::MessagePart;
use parse::Input;

mod messages {
	pub struct MessagePart {
		data: Vec<Byte>,
	}

	use super::data::*;
	use super::Byte;

	pub struct CommandMessage {
		id: Option<Int>,
		command_name: String,
		arguments: Collection,
	}

	pub struct Line {
		data: Vec<Byte>,
	}

	impl SingleLine for Line {}

	// TODO: Support for standard error messages?

	pub enum Content {
		Collection(Collection),
		Response(MultilineList<Line>),
		ErrorMessage(MultilineList<List<String>>),
	}

	pub struct ResponseMessage {
		id: Option<Int>,
		content: Content,
	}
}

mod command {
	use super::data::*;

	// TODO: Macros

	struct Command {
		name: String,
	}
}
