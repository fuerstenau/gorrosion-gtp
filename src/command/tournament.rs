//! Commands needed to play under tournament conditions.

use super::*;

/// Configure the time mode.
///
/// When given the arguments `(main_time, byo_yomi_time, stones)`
/// set the time mode to be Canadian byo yomi with the obvious parameters
/// where time is measured in seconds.
/// When `byo_yomi_time > 0` and `stones == 0`,
/// this is defined to mean no time limits.
///
/// # Effects
/// > The time settings are changed.
///
/// # Fails
/// * Syntax error
///
/// # Comments
/// > The interpretation of the parameters is discussed in section 4.2.
/// > The engine must accept the requested values.
/// > This command gives no provision for negotiation of the time settings.
pub fn time_settings() -> Command {
	unimplemented!()
	//	command!("time_settings" => ({int int int}) ->
}

/// Give information about the current time status.
///
/// The controller tells the engine
/// how much time one colour has left
/// on their clock / in their current byo yomi period
/// in the format `(colour, time_left, stones_left)`.
///
/// # Fails
/// * Syntax error
///
/// # Comments
/// > While the main time is counting,
/// > the number of remaining stones is given as 0.
pub fn time_left() -> Command {
	unimplemented!()
	//	command!("time_left" => ({color int int}) -> (none))
}
