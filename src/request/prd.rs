//! The PRD
//!
//! The product requirements document the ticket links, read for the one fact
//! the simulation shows of it: the use case.
//!
//! Definition only: the `Prd` type: the link it was read from and its use case.
//! Its functions are the files in `prd/`.
//!
//! ## Rules
//!
//! - The same document gives the same fact every time: no model reads it.

pub mod card;
pub mod line;
pub mod read;

use crate::request::link::Link;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prd {
	pub link: Link,
	pub use_case: String,
}
