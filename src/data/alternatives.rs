//! The so-called specification is unclear
//! on the matter of alternatives of compound types.
//! For now, we'll do the simpler thing.

use super::*;

pub type Value = simple_entity::Value;

pub struct Type {
	left: simple_entity::Type,
	right: simple_entity::Type,
}
