//! One line of the session
//!
//! Every line nsx prints is one line of the simulation, known by the id the
//! simulation gives it. The goldens list each line the same way: its id, its
//! kind and its text.
//!
//! Definition only: the `Line` type, which carries the line's id, its kind
//! and its shape, the words its layout needs. Its functions are the files in
//! `line/`.
//!
//! ## Rules
//!
//! - A line is one UTF-8 line of English, ended by a newline, with no
//!   trailing space.
//! - Items inside a value are separated by ` · `.
//! - Numbers print without thousands separators (`1450m`, `2304 MiB`). Money
//!   and token figures follow spec 0056 (`$25.00`, `3.5M`, `250K`).
//! - The words of a line belong to the folder that owns it, never to this
//!   one.
//!
//! ## Open
//!
//! - Where the catalog lives. The spec keeps one catalog of the lines, keyed
//!   by the simulation's line ids. This draft has no file for it: each
//!   line's words sit in its owner's `line.rs`.
//!
//! Spec 0077.

pub mod comment;
pub mod render;

use crate::session::{kind::Kind, shape::Shape};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line {
	pub id: String,
	pub kind: Kind,
	pub shape: Shape,
}
