use super::super::messages::WriteGTP;
use super::*;
use std::io;

pub enum Value {
	SimpleEntity(simple_entity::Value),
	Collection(collection::Value),
	List(list::Value),
	Alternatives(alternatives::Value),
}

impl From<simple_entity::Value> for Value {
	fn from(t: simple_entity::Value) -> Value {
		Value::SimpleEntity(t)
	}
}

impl From<list::Value> for Value {
	fn from(t: list::Value) -> Value {
		Value::List(t)
	}
}

impl From<collection::Value> for Value {
	fn from(t: collection::Value) -> Value {
		Value::Collection(t)
	}
}

impl From<alternatives::Value> for Value {
	fn from(t: alternatives::Value) -> Value {
		Value::Alternatives(t)
	}
}

impl WriteGTP for Value {
	fn write_gtp(&self, f: &mut impl io::Write) -> io::Result<()> {
		match self {
			Value::SimpleEntity(v) => v.write_gtp(f),
			Value::Collection(v) => v.write_gtp(f),
			Value::List(v) => v.write_gtp(f),
			Value::Alternatives(v) => v.write_gtp(f),
		}
	}
}

#[derive(Clone, Debug)]
pub enum Type {
	SimpleEntity(simple_entity::Type),
	Collection(collection::Type),
	List(list::Type),
	Alternatives(alternatives::Type),
}

impl From<simple_entity::Type> for Type {
	fn from(t: simple_entity::Type) -> Type {
		Type::SimpleEntity(t)
	}
}

impl From<list::Type> for Type {
	fn from(t: list::Type) -> Type {
		Type::List(t)
	}
}

impl From<collection::Type> for Type {
	fn from(t: collection::Type) -> Type {
		Type::Collection(t)
	}
}

impl From<alternatives::Type> for Type {
	fn from(t: alternatives::Type) -> Type {
		Type::Alternatives(t)
	}
}

impl Data for Value {
	type Type = Type;

	fn parse<'a, I: Input<'a>>(i: I, t: &Self::Type) -> IResult<I, Self> {
		match t {
			Type::SimpleEntity(t) => {
				let res = simple_entity::Value::parse(i, t);
				res.map(|(i, v)| (i, From::from(v)))
			},
			Type::Collection(t) => {
				let res = collection::Value::parse(i, t);
				res.map(|(i, v)| (i, From::from(v)))
			},
			Type::List(t) => {
				let res = list::Value::parse(i, t);
				res.map(|(i, v)| (i, From::from(v)))
			},
			Type::Alternatives(t) => {
				let res = alternatives::Value::parse(i, t);
				res.map(|(i, v)| (i, From::from(v)))
			},
		}
	}
}
