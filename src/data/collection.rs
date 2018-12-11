use super::*;

// This additional layer of indirection brought to you by
// the weird semi-support of Rust for enums.
// If we want to export it publicly under some name,
// we have to use this name in the initial declaration already.
/// A heterogenous tuple.
///
/// The spec says
/// > [A pair `(x, y)` where]
/// > `x` and `y` may be any combination of simple entities.
/// > The construction can be generalized
/// > to any fixed number of entities.
///
/// I was not able to determine
/// whether tuples of length 0 and 1 are allowed
/// so we do the simpler thing and include them in our abstraction.
// TODO: Convert to and from tuples.
pub enum Collection {
	Empty,
	Collection(simple_entity::Value, Box<Collection>),
}

pub type Value = Collection;

pub enum Type {
	Empty,
	Collection(simple_entity::Type, Box<Type>),
}
