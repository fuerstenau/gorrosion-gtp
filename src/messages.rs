//! In GTP, commands are sent one way (Controller -> Engine)
//! and responses another (Engine -> Controller).
//! Representations of both of these,
//! ready to be sent out, are provided here.

use super::gtp_type::*;
use std::io;

/// One of the two most-relevant traits of all:
/// Write your content according to the rules of to some manner of output.
pub trait WriteGTP {
	fn write_gtp(&self, &mut impl io::Write) -> io::Result<()>;
}

/// A command that can be sent to some engine via `write_gtp`.
/// Bring your own engine.
pub struct Command {
	id: Option<Int>,
	command_name: String,
	arguments: List,
}

impl WriteGTP for Command {
	fn write_gtp(&self, f: &mut impl io::Write) -> io::Result<()> {
		if let Some(id) = &self.id {
			id.write_gtp(f)?;
			f.write_all(b" ")?;
		};
		self.command_name.write_gtp(f)?;
		if !self.arguments.is_empty() {
			f.write_all(b" ")?;
			self.arguments.write_gtp(f)?;
		};
		Ok(())
	}
}

// TODO: Support for standard error messages?

/// A response that can be sent to some controller via `write_gtp`,
/// ideally matching a previously received command.
/// Bring your own controller.
pub struct Response {
	id: Option<Int>,
	success: bool,
	content: MultilineList,
}

impl WriteGTP for Response {
	fn write_gtp(&self, f: &mut impl io::Write) -> io::Result<()> {
		unimplemented!()
	}
}
