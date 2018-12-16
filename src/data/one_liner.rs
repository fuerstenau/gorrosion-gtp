use super::super::messages::WriteGTP;
use super::*;
use std::io;

pub enum Value {
	SimpleEntity(simple_entity::Value),
	Collection(collection::Value),
	List(list::Value),
	Alternatives(alternatives::Value),
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
