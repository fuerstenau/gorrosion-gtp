//! Abstract description of a command
//! as specified in some external document.
//!
//! The submodules (will) provide all the commands specified for GTP 2,
//! non-standard extensions are planned for inclusion.
//! When a command has an effect or may fail,
//! this should be noted in the documentation.

use super::data;
use super::gtp_type::*;
use super::input;

/* Standard Errors
unknown command
unacceptable size
illegal move
cannot undo
*/

/// Abstract representation of a command that may be supported by an engine.
/// Contains the name of the command
/// as well as the argument and result type specifications.
#[derive(Debug)]
pub struct Command {
	name: String,
	arguments: <List as data::Data>::Type,
	response: <MultilineList as data::Data>::Type,
}

use data::alternatives::Type as Alt;
use data::collection::Type as Cl;
use data::list::Type as Lst;
use data::multiline_list::Type as ML;
use data::simple_entity::Type as SE;

macro_rules! type_description {
	( none ) => {
		Cl::Empty
	};
	( int ) => {
			SE::Int
	};
	( float ) => {
		SE::Float
	};
	( string ) => {
		SE::String
	};
	( vertex ) => {
		SE::Vertex
	};
	( color ) => {
		SE::Color
	};
	( move ) => {
		SE::Motion
	};
	( boolean ) => {
		SE::Boolean
	};
	( $f:tt | $s:tt ) => {
		<Alt as From<_>>::from((
			type_description!($f),
			type_description!($s),
			))
	};
	( $t:tt * ) => {
		<Lst as From<_>>::from(type_description!($t))
	};
	( $t:tt & ) => {
		<ML as From<_>>::from(type_description!($t))
	};
	( $t:tt * & ) => {
		<ML as From<_>>::from(type_description!($t *))
	};
}

// Rustfmt has a weird bug regarding the last lines of this macro.
#[rustfmt::skip]
macro_rules! str_to_gtp {
	( $e:expr ) => {{
		let s = $e;
		let i = input::engine::Input::from(s.as_bytes());
		let t = data::string::Type::default();
		let (_, r) = <String as data::Data>::parse(i, &t).unwrap();
		r
	}};
}

// TODO: Export this macro.
/// Specify a new command.
///
/// The macro takes three arguments, separated by semicola.
/// The first is the name of the command, e.g. as string constant,
/// the other two are the type specification
/// of the arguments and the response, respectively.
/// The syntax for specfication of types is not final
/// and due to restrictions of Rusts macro system
/// (There appears to be no way to let `*` be a part of the macros syntax
/// since it is part of the `macro_rules!` syntax.)
/// not exactly as in the spec.
//
// # Examples
//
// ```
// # #[macro_use]
// # use gorrosion_gtp::command;
// # use gorrosion_gtp::command::Command;
// let c: Command = command!("name"; none; [ string ]);
// ```
macro_rules! command {
	($name:expr => ($($args:tt)*) -> ($($resp:tt)*)) => {{
		let name = str_to_gtp!($name);
		let arguments = From::from(type_description!($($args)*));
		let response = From::from(type_description!($($resp)*));
		Command { name, arguments, response }
	}};
}

pub mod administrative;
pub mod core_play;
pub mod setup;
pub mod tournament;
