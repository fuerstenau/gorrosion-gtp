//! (Unpublished)

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

macro_rules! type_description {
	( none ) => { Cl::Empty };
	( int ) => { SE::Int };
	( float ) => { SE::Float };
	( string ) => { SE::String };
	( vertex ) => { SE::Vertex };
	( color ) => { SE::Color };
	( move ) => { SE::Motion };
	( boolean ) => { SE::Boolean };
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

macro_rules! command {
	($name:expr; $args:tt; $resp:tt) => {{
		let name = str_to_gtp!($name);
		let arguments = From::from(type_description!($args));
		let response = From::from(type_description!($resp));
		Command { name, arguments, response }
	}}
}

pub fn protocol_version() -> Command {
	command!("protocol_version"; none; int)
}
