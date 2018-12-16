//! The types described by the GTP specification.
//! They have an ever growing collection
//! of useful methods to manipulate them.
//! The most interesting should currently be the conversions
//! to more usual data types of Rust.

use super::data::*;

/// An unsigned integer of 31 bits.
///
/// The spec says:
/// > An `int` is an unsigned integer
/// > in the interval *$0 <= x <= 2^{31} - 1$*.
///
/// Consequently, it will most easily be manipulated
/// by casting it to `u32` or `i32` and back.
pub type Int = int::Value;

/// A 32 bit IEEE 754 float.
///
/// Most easily manipulated by casting it to `f32` and back.
pub type Float = float::Value;

/// A single word.
///
/// The spec says:
/// > A string is a sequence of printable, non-whitespace characters.
/// > Strings are case sensitive.
///
/// Unfortunately, any ASCII compatible encoding is allowed by GTP
/// and thus there is no straightforward way
/// to reliably cast this into anything but a `Vec<Byte>`.
pub type String = string::Value;

/// A position where to play.
/// Either coordinates of a vertex on the board or `Pass`.
///
/// The spec says
/// > A vertex is a board coordinate
/// > consisting of one letter and one number
/// > [or `Pass`].
/// Vertices are not case sensitive.
/// Examples: “B13”, “j11”.
pub type Vertex = vertex::Value;

/// The colours of the two opponents, either `Black` or `White`.
pub type Color = self::color::Value;

/// A move that can be made.
///
/// A `Move` comprises a `Color` and a `Vertex`,
/// representing that a play for that colour is made at that `Vertex`.
/// Resignation can not be represented as a `Move`.
pub type Move = motion::Value;

/// A boolean value, either `True` or `False`.
///
/// Most easily used, resp. provided, by casting to, resp. from, `bool`.
pub type Boolean = self::boolean::Value;

/// A heterogenous tuple.
///
/// The spec says
/// > [A pair `(x, y)` where]
/// > `x` and `y` may be any combination of simple entities.
/// > The construction can be generalized
/// > to any fixed number of entities.
///
/// I was not able to determine
/// whether tuples of length 0 and 1 are allowed
/// so we do the simpler thing and include them in our abstraction.
pub type Collection = self::collection::Value;

/// A homogeneous list of `Collection`s.
///
/// The spec says:
/// > An `x*` is a […] list of entities of type `x`,
/// > where `x` may be any of the [simple entities or a `Collection`].
/// > The list can have an arbitrary number of elements […].
///
/// For ease of use we deviate from the specification
/// and allow only lists of collections.
/// This is not an actual limitation
/// as there is a canonical correspondence
/// between simple entities and 1-tuples
/// and hence lists of simple entities
/// can be modeled faithfully as lists of singleton collections.
pub type List = list::Value;

/// An (untagged) union of two types.
///
/// The spec says:
/// > An `x|y` is either an `x` or a `y`.
///
/// Since all examples within the spec
/// are of alternatives of two simple entities
/// only those are currently supported.
pub type Alternatives = alternatives::Value;

/// A homogenous list containing elements of an arbitrary type
/// (except multiline lists).
pub type MultilineList = multiline_list::Value;
