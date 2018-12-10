use super::gtp_type::*;
use super::Byte;

pub struct MessagePart {
	data: Vec<Byte>,
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
