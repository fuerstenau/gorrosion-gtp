//! The standard comments to set up games.

use super::*;

/// Configure boardsize.
///
/// # Effects
/// > The board size is changed.
/// > The board configuration, number of captured stones, and move history
/// > become arbitrary.
///
/// # Fails
/// * Syntax error
/// * Engine can not handle the new size (`"unacceptable size"`)
///
/// # Comments
/// > In GTP version 1 this command also did the work of `clear_board`.
/// > This may or may not be true for implementations of GTP version 2.
/// > Thus the controller must call `clear_board` explicitly.
/// > Even if the new board size is the same as the old one,
/// > the board configuration becomes arbitrary.
pub fn boardsize() -> Command {
	command!("boardsize" => (int) -> (none))
}

/// Reset game state.
///
/// # Effects
/// > The board is cleared,
/// > the number of captured stones is reset to zero for both colors
/// > and the move history is reset to empty.
pub fn clear_board() -> Command {
	command!("clear_board" => (none) -> (none))
}

/// Adjust komi value.
///
/// # Effects
/// > Komi is changed.
///
/// # Fails
/// * Syntax error
///
/// # Comments
/// >  The engine must accept the komi even if it should be ridiculous.
pub fn komi() -> Command {
	command!("komi" => (float) -> (none))
}

/// Place fixed handicap.
///
/// Places the specified number of handicap stones
/// and returns the list of vertices on which the stones have been placed.
///
/// Once integration with the [gorrosion crate](crates.io/gorrosion)
/// has advanced far enough, this can be automated.
/// In particular, the data from Section 4.1.1 will be known to this crate.
///
/// # Effects
/// > Handicap stones are placed on the board
/// > according to the specification in section 4.1.1.
///
/// # Fails
/// * Syntax error
/// * Invalid number of stones
/// * Board is not empty
///
/// # Comments
/// > This command is only valid if the board is empty.
/// > See section 4.1.1 for valid number of handicap stones.
/// > The handicap stones are *not* included in the move history
pub fn fixed_handicap() -> Command {
	command!("fixed_handicap" => (int) -> (vertex*))
}

/// Engine: Place your own free hadicap.
///
/// # Effects
/// > Handicap stones are placed on the board
/// > on the vertices the engine prefers.
/// > See also section 4.1.2.
///
/// The relevant part of Section 4.1.2 is
/// > [W]hen the number of handicap stones becomes very high
/// > there is no benefit in additional stones.
/// > Therefore, when asked to choose handicap placement,
/// > an engine is allowed to return a smaller number of stones than requested.
/// > This provision should only be used
/// > if the requested number of stones is so high
/// > that a smaller number of stones is believed to guarantee
/// > that the engine cannot possibly lose against any opponent.
///
/// # Fails
/// * Syntax error
/// * Invalid number of stones (At least two)
/// * Board not empty
/// * Bad vertex list
///   (I do not understand this one.
///   It might have been mistakenly copied from `set_free_handicap`.)
///
/// # Comments
/// > This command is only valid if the board is empty.
/// > The engine may place fewer than the requested number of stones
/// > on the board
/// > under certain circumstances, as discussed in section 4.1.2.
/// > The controller can check this
/// > by counting the number of vertices in the response.
/// > The handicap stones are *not* included in the move history.
/// > Vertices must not be repeated or include “pass”.
pub fn place_free_handicap() -> Command {
	command!("place_free_handicap" => (int) -> (vertex*))
}

/// Place the free handicap someone else chose.
///
/// # Effects
/// > Handicap stones are placed on the vertices as requested.
///
/// # Fails
/// * Syntax error
/// * Board not empty
/// * Bad vertex list
///
/// # Comments
/// > This command is only valid if the board is empty.
/// > The list must have at least two elements
/// > and no more than the number of board vertices minus one.
/// > The engine must accept the handicap placement.
/// > The handicap stones are *not* included in the move history.
/// >  Vertices must not be repeated or include “pass”.
pub fn set_free_handicap() -> Command {
	command!("set_free_handicap" => (vertex*) -> (none))
}
