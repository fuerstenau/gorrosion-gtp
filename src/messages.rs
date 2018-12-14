use super::gtp_type::*;
use std::io;

pub trait Writable {
	fn write_gtp(&self, &mut impl io::Write) -> io::Result<()>;
}

pub struct CommandMessage {
	id: Option<Int>,
	command_name: String,
	arguments: Collection,
}

// TODO: Support for standard error messages?

pub struct ResponseMessage {
	id: Option<Int>,
	success: bool,
	content: MultilineList,
}
