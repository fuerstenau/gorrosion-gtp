use super::gtp_type::*;
use super::Byte;
use std::io;

pub trait Writable {
	fn write_gtp(&self, &mut impl io::Write) -> io::Result<()>;
}

pub struct MessagePart {
	msg: Vec<Byte>,
}

impl<T> From<T> for MessagePart
where
	T: Writable
{
	fn from(t: T) -> MessagePart {
		let mut msg = Vec::new();
		t.write_gtp(&mut msg);
		MessagePart { msg }
	}
}

pub struct CommandMessage {
	id: Option<Int>,
	command_name: String,
	arguments: Collection,
}

// TODO: Support for standard error messages?

pub enum Content {
	Collection(Collection),
	Response(MultilineList),
	ErrorMessage(MultilineList),
}

pub struct ResponseMessage {
	id: Option<Int>,
	content: Content,
}
