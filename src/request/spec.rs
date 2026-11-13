//! The spec
//!
//! The spec the ticket links, read for the components it names and for how
//! many acceptance criteria it lists.
//!
//! Definition only: the `Spec` type: the link it was read from, each component
//! it has `Named`, in its order, and its count of acceptance criteria. Its
//! functions are the files in `spec/`.
//!
//! ## Rules
//!
//! - The component paths, in the spec's order and each with the port the spec
//!   names beside it, are handed on to `components/`, which inspects them.

pub mod card;
pub mod line;
pub mod read;

use crate::request::{link::Link, named::Named};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spec {
	pub link: Link,
	pub components: Vec<Named>,
	pub criteria: u64,
}
