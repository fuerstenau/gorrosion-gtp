use super::gtp_type::*;
use super::Byte;

pub struct MessagePart {
	msg: Vec<Byte>,
}

impl From<Int> for MessagePart {
	fn from(int: Int) -> MessagePart {
		let msg = Vec::from(u32::from(int).to_string().as_bytes());
		MessagePart { msg }
	}
}

impl From<Float> for MessagePart {
	fn from(f: Float) -> MessagePart {
		let msg = Vec::from(f32::from(f).to_string().as_bytes());
		MessagePart { msg }
	}
}

impl From<String> for MessagePart {
	fn from(str: String) -> MessagePart {
		let msg = str.into();
		MessagePart { msg }
	}
}

impl From<Vertex> for MessagePart {
	fn from(vert: Vertex) -> MessagePart {
		let msg: Vec<Byte> = match vert {
			Vertex::Pass => b"pass".to_vec(),
			Vertex::Coord(letter, number) => {
				let mut num = Vec::from(
					number.to_string().as_bytes(),
				);
				let mut msg = Vec::with_capacity(num.len() + 1);
				msg.push(letter as Byte);
				msg.append(&mut num);
				msg
			}
		};
		MessagePart { msg }
	}
}

impl From<Color> for MessagePart {
	fn from(col: Color) -> MessagePart {
		let msg = match col {
			Color::Black => b"Black".to_vec(),
			Color::White => b"White".to_vec(),
		};
		MessagePart { msg }
	}
}

impl From<Move> for MessagePart {
	fn from(m: Move) -> MessagePart {
		let mut msg = MessagePart::from(*m.color()).msg;
		msg.extend(b" ");
		msg.append(&mut MessagePart::from(*m.vertex()).msg);
		MessagePart { msg }
	}
}

impl From<Boolean> for MessagePart {
	fn from(b: Boolean) -> MessagePart {
		let msg = match b {
			Boolean::False => b"false".to_vec(),
			Boolean::True => b"true".to_vec(),
		};
		MessagePart { msg }
	}
}

impl From<Collection> for MessagePart {
	fn from(c: Collection) -> MessagePart {
		let msg = match c {
			Collection::Empty => Vec::new(),
			Collection::Collection(head, tail) => {
				let mut msg = b" ".to_vec();
				msg.append(&mut MessagePart::from(head).msg);
				msg.append(&mut MessagePart::from(*tail).msg);
				msg
			}
		};
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
