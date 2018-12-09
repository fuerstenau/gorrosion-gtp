#![feature(try_from)]

#[macro_use]
extern crate nom;

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
