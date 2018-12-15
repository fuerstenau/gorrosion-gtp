//! The commands from the specification that are listed as administrative.

use super::*;

/// > For this specification 2.
pub fn protocol_version() -> Command {
	command!("protocol_version"; none; int)
}

/// >  E.g. “GNU Go”, “GoLois”, “Many Faces of Go”.
/// > The name does not include any version information,
/// > which is provided by the version command.
pub fn name() -> Command {
	command!("name"; none; [ string ])
}

/// > E.g. “3.1.33”, “10.5”.
/// > Engines without a sense of version number should return the empty string.
pub fn version() -> Command {
	command!("version"; none; [ string ])
}

/// > The protocol makes no distinction
/// > between unknown commands and known but unimplemented ones.
/// > Do not declare a command as known if it is known not to work.
pub fn known_command() -> Command {
	command!("known_command"; string; boolean)
}
