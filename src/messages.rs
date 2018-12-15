use super::gtp_type::*;
use std::io;

pub trait WriteGTP {
	fn write_gtp(&self, &mut impl io::Write) -> io::Result<()>;
}

pub struct Command {
	id: Option<Int>,
	command_name: String,
	arguments: Collection,
}

impl WriteGTP for Command {
	fn write_gtp(&self, f: &mut impl io::Write) -> io::Result<()> {
		if let Some(id) = &self.id {
			id.write_gtp(f)?;
			f.write_all(b" ")?;
		};
		self.command_name.write_gtp(f)?;
		if !self.arguments.empty() {
			f.write_all(b" ")?;
			self.arguments.write_gtp(f)?;
		};
		Ok(())
	}
}

// TODO: Support for standard error messages?

pub struct Response {
	id: Option<Int>,
	success: bool,
	content: MultilineList,
}
