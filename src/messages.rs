//! In GTP, commands are sent one way (Controller -> Engine)
//! and responses another (Engine -> Controller).
//! Representations of both of these,
//! ready to be sent out, are provided here.

use super::gtp_type::*;
use std::io;

/// One of the two most-relevant traits of all:
/// Write a Rust object as GTP.
pub trait WriteGTP {
	/// Write the GTP representation of `self` to the sink.
	///
	/// # Example
	///
	/// ```
	/// # #![feature(try_from)]
	/// # use std::convert::TryFrom;
	/// # use gorrosion_gtp::messages::WriteGTP;
	/// # use gorrosion_gtp::gtp_type;
	/// let i = gtp_type::Int::try_from(42).unwrap();
	/// let mut out: Vec<u8> = Vec::with_capacity(2);
	/// i.write_gtp(&mut out);
	/// assert_eq!(&out, b"42");
	/// ```
	fn write_gtp(&self, &mut impl io::Write) -> io::Result<()>;
}

/// A command that can be sent to some engine via `write_gtp`.
/// Bring your own engine.
#[derive(Debug)]
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
#[derive(Debug)]
pub struct Response {
	id: Option<Int>,
	success: bool,
	content: MultilineList,
}

impl WriteGTP for Response {
	fn write_gtp(&self, f: &mut impl io::Write) -> io::Result<()> {
		if self.success {
			write!(f, "=")?;
		} else {
			write!(f, "?")?;
		};
		if let Some(int) = &self.id {
			int.write_gtp(f)?;
		}
		write!(f, " ")?;
		self.content.write_gtp(f)
	}
}
