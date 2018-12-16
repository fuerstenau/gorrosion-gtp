//! Due to rustdoc lacking an [important feature][alias-doc],
//! you will have to build the documentation including private items
//! to be able to look up all the functions and traits
//! that are defined by type aliases.
//! If you are viewing this on [docs.rs][docs-rs],
//! the private items should already be included.
//!
//! [alias-doc]: https://github.com/rust-lang/rust/issues/32077
//! [docs-rs]: https://docs.rs/gorrosion-gtp

#![feature(stmt_expr_attributes)]
#![feature(try_from)]
// TODO: Disable once all the code lives
#![allow(dead_code)]

#[macro_use]
extern crate nom;

/// Despite its name,
/// GTP feels like a binary protocol
/// and we will treat it as such.
/// Consequently, all “text” is a sequence of bytes.
/// To emphasize that these bytes are not used for their numerical value
/// but rather their property of “most generic kind of data”
/// we will explicitly refer to them as `Byte` instead of `u8`.
type Byte = u8;

mod data;
mod input;

pub mod command;
pub mod gtp_type;
pub mod messages;
