use super::*;

// This additional layer of indirection brought to you by
// the weird semi-support of Rust for enums.
// If we want to export it publicly under some name,
// we have to use this name in the initial declaration already.
pub enum Collection {
	Empty,
	Collection(simple_entity::Value, Box<Collection>),
	List(list::Value),
}

pub type Value = Collection;

pub enum Type {
	Empty,
	Collection(simple_entity::Type, Box<Type>),
	List(list::Type),
}
