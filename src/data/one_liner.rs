use super::{alternatives, collection, list, simple_entity};

pub enum Value {
	SimpleEntity(simple_entity::Value),
	Collection(collection::Value),
	List(list::Value),
	Alternatives(alternatives::Value),
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
