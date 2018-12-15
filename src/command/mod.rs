//! Abstract description of a command
//! as specified in some external document.
//!
//! The submodules (will) provide all the commands specified for GTP 2,
//! non-standard extensions are planned for inclusion.
//! When a command has an effect or may fail,
//! this should be noted in the documentation.

use super::gtp_type::*;
use super::data;
use super::input;

/* Standard Errors
unknown command
*/

pub struct Command {
	name: String,
	arguments: <Collection as data::Typed>::Type,
	response: <MultilineList as data::Typed>::Type,
}

use data::simple_entity::Type as SE;
use data::collection::Type as Cl;
use data::list::Type as Lst;

macro_rules! type_description {
	( none ) => { Cl::Empty };
	( int ) => { SE::Int };
	( float ) => { SE::Float };
	( string ) => { SE::String };
	( vertex ) => { SE::Vertex };
	( color ) => { SE::Color };
	( move ) => { SE::Motion };
	( boolean ) => { SE::Boolean };
	( [ $t:tt ] ) => { <Lst as From<_>>::from(type_description!($t)) };
}

macro_rules! str_to_gtp {
	( $e:expr ) => {{
		let s = $e;
		let i = input::engine::Input::from(s.as_bytes());
		let t = data::string::Type::default();
		let (_, r) = <String as data::Data>::parse(i, t).unwrap();
		r
	}}
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
	($name:expr; $args:tt; $resp:tt) => {{
		let name = str_to_gtp!($name);
		let arguments = From::from(type_description!($args));
		let response = From::from(type_description!($resp));
		Command { name, arguments, response }
	}}
}

pub mod administrative;
