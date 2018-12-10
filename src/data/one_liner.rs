use super::{simple_entity, collection, list, alternatives};

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
