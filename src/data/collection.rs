use super::super::messages::WriteGTP;
use super::*;
use std::io;

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

	fn write_space_gtp(&self, f: &mut impl io::Write) -> io::Result<()> {
		match self {
			Value::Empty => Ok(()),
			Value::Collection(head, tail) => {
				write!(f, " ")?;
				head.write_gtp(f)?;
				tail.write_space_gtp(f)
			}
		}
	}
}

impl WriteGTP for Value {
	fn write_gtp(&self, f: &mut impl io::Write) -> io::Result<()> {
		match self {
			Value::Empty => Ok(()),
			Value::Collection(head, tail) => {
				head.write_gtp(f)?;
				tail.write_space_gtp(f)
			}
		}
	}
}

#[derive(Clone, Debug)]
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

impl HasType<Type> for Value {
	fn has_type(&self, t: &Type) -> bool {
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
	type Type = Type;

	fn parse<'a, I: Input<'a>>(i: I, t: &Self::Type) -> IResult<I, Self> {
		#[rustfmt::skip]
		match t {
			Type::Empty => Ok((i, Value::Empty)),
			Type::Collection(head_t, tail_t) => {
				do_parse!(i,
					tag!(" ") >>
					head: parse_gtp!(head_t) >>
					tail: parse_gtp!(&**tail_t) >>
					(Value::new(head, tail))
				)
			}
		}
	}
}
