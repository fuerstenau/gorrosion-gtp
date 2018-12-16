//! The so-called specification is unclear
//! on the matter of alternatives of compound types.
//! For now, we'll do the simpler thing.

use super::*;

pub type Value = simple_entity::Value;

pub struct Type {
	first: simple_entity::Type,
	second: simple_entity::Type,
}

impl From<(simple_entity::Type, simple_entity::Type)> for Type {
	fn from(pair: (simple_entity::Type, simple_entity::Type)) -> Self {
		let (first, second) = pair;
		Type { first, second }
	}
}
