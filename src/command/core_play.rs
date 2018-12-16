//! The core commands to play a game.

use super::*;

/// Execute the specified move.
///
/// # Effects
/// > A stone of the requested color is played at the requested vertex.
/// > The number of captured stones is updated if needed
/// > and the move is added to the move history.
///
/// # Fails
/// * Syntax error
/// * Illegal move (`"illegal move"`)
///
/// # Comments
/// >  Consecutive moves of the same color are not considered illegal
/// > from the protocol point of view.
pub fn play() -> Command {
	command!("play" => (move) -> (none))
}

/// Decide on a move to play.
///
/// # Effects
/// >  A stone of the requested color is played where the engine chooses.
/// > The number of captured stones is updated if needed
/// > and the move is added to the move history.
///
/// # Comments
/// > Notice that “pass” is a valid vertex and should be returned
/// > if the engine wants to pass.
/// > Use “resign” if you want to give up the game.
/// > The controller is allowed to use this command for either color,
/// > regardless who played the last move.
pub fn genmove() -> Command {
	command!("genmove" => (color) -> (vertex|string))
}

/// Undo the last move.
///
/// # Effects
/// > The board configuration and the number of captured stones
/// > are reset to the state before the last move.
/// > The last move is removed from the move history.
///
/// # Fails
/// * Engine is unable to undo the last move (`"cannot undo"`)
///
/// # Comments
/// > If you want to take back multiple moves, use this command multiple times.
/// > The engine may fail to undo if the move history is empty
/// > or if the engine only maintains a partial move history,
/// > which has been exhausted by previous undos.
/// > It is never possible to undo handicap placements.
/// > Use `clear_board` if you want to start over.
/// > An engine which never is able to undo should not include this command
/// > among its known commands.
pub fn undo() -> Command {
	command!("undo" => (none) -> (none))
}
