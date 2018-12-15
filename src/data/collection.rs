use super::*;
use std::io;
use super::super::messages::WriteGTP;

// TODO: Convert to and from tuples.
pub enum Value {
	Empty,
	Collection(simple_entity::Value, Box<Value>),
}

impl Value {
	fn new(head: simple_entity::Value, tail: Value) -> Value {
		Value::Collection(head, Box::new(tail))
	}

	pub fn is_empty(&self) -> bool {
		match self {
			Value::Empty => true,
			Value::Collection(_, _) => false,
		}
	}
}

impl WriteGTP for Value {
	fn write_gtp(&self, f: &mut impl io::Write) -> io::Result<()> {
		unimplemented!()
	}
}

pub enum Type {
	Empty,
	Collection(simple_entity::Type, Box<Type>),
}

impl From<simple_entity::Type> for Type {
	fn from(t: simple_entity::Type) -> Type {
		let empty = Box::new(Type::Empty);
		Type::Collection(t, empty)
	}
}

impl Typed for Value {
	type Type = Type;
}

impl HasType for Value {
	fn has_type(&self, t: &Self::Type) -> bool {
		match (self, t) {
			(Value::Empty, Type::Empty) => true,
			(
				Value::Collection(val, col),
				Type::Collection(head, tail),
			) => val.has_type(head) & col.has_type(&*tail),
			_ => false,
		}
	}
}

impl Data for Value {
	fn parse<'a, I: Input<'a>>(i: I, t: Self::Type) -> IResult<I, Self> {
		match t {
			Type::Empty => Ok((i, Value::Empty)),
			Type::Collection(head, tail) => {
				let parse_head = |i| {
					simple_entity::Value::parse(i, head)
				};
				#[rustfmt::skip]
				let parse_tail = |i| {
					collection::Value::parse(i, *tail)
				};
				#[rustfmt::skip]
				do_parse!(i,
					tag!(" ") >>
					head: call!(parse_head) >>
					tail: call!(parse_tail) >>
					(Value::new(head, tail))
				)
			}
		}
	}
}
