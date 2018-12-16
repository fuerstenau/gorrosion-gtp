//! The commands from the specification that are listed as administrative.

use super::*;

/// Which version of the protocol is spoken?
///
/// # Comments
/// > For this specification 2.
pub fn protocol_version() -> Command {
	command!("protocol_version" => (none) -> (int))
}

/// What is the name of the engine?
///
/// # Comments
/// >  E.g. “GNU Go”, “GoLois”, “Many Faces of Go”.
/// > The name does not include any version information,
/// > which is provided by the version command.
pub fn name() -> Command {
	command!("name" => (none) -> (string*))
}

/// What is the version of the engine?
///
/// # Comments
/// > E.g. “3.1.33”, “10.5”.
/// > Engines without a sense of version number should return the empty string.
pub fn version() -> Command {
	command!("version" => (none) -> (string*))
}

/// Does the engine know this command?
///
/// # Comments
/// > The protocol makes no distinction
/// > between unknown commands and known but unimplemented ones.
/// > Do not declare a command as known if it is known not to work.
pub fn known_command() -> Command {
	command!("known_command" => (string) -> (boolean))
}

/// Which commands does the engine know?
///
/// # Comments
/// > Include all known commands,
/// > including required ones and private extensions.
pub fn list_commands() -> Command {
	command!("list_commands" => (none) -> (string&))
}

/// End the communication.
///
/// # Effects
/// > The session is terminated and the connection is closed.l
///
/// # Comments
/// > The full response of this command
/// > must be sent before the engine closes the connection.
/// > The controller must receive the response
/// > before the connection is closed on its side.
pub fn quit() -> Command {
	command!("quit" => (none) -> (none))
}
