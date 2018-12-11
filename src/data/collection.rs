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

impl Value {
	fn new(head: simple_entity::Value, tail: Value) -> Value {
		Collection::Collection(head, Box::new(tail))
	}
}

pub enum Type {
	Empty,
	Collection(simple_entity::Type, Box<Type>),
}

impl Typed for Value {
	type Type = Type;
}

impl HasType for Value {
	fn has_type(&self, t: &Self::Type) -> bool {
		match (self, t) {
			(Collection::Empty, Type::Empty) => true,
			(
				Collection::Collection(val, col),
				Type::Collection(head, tail),
			) => val.has_type(head) & col.has_type(&*tail),
			_ => false,
		}
	}
}

impl Data for Value {
	fn parse(i: Input, t: Self::Type) -> IResult<Input, Self> {
		match t {
			Type::Empty => Ok((i, Collection::Empty)),
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
					head: call!(parse_head) >>
					tag!(b" ") >>
					tail: call!(parse_tail) >>
					(Value::new(head, tail))
				)
			}
		}
	}
}
