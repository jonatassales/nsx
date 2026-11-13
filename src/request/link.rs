//! One link of the ticket
//!
//! One document the ticket links, as the plan names and finds it.
//!
//! Definition only: the `Link` type: the document's kind, its file name, its
//! short id and its project-relative path. Its function is the file in
//! `link/`.
//!
//! ## Rules
//!
//! - The short id is what the lines show, the file name what the card shows.
//! - `Links` holds one link per kind.

pub mod locate;

use std::path::PathBuf;

use crate::request::kind::Kind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
	pub kind: Kind,
	pub name: String,
	pub short: String,
	pub path: PathBuf,
}
