//! The design doc
//!
//! The design doc the ticket links, read for what the team set in it: the
//! envelope the runtime runs in and the budget of a run's model calls.
//!
//! Definition only: the `Design` type: the link it was read from, the envelope
//! in millicores and MiB, and the budget per run in cents. Its functions are
//! the files in `design/`.
//!
//! ## Rules
//!
//! - Whole units, as plain integers: the envelope in millicores and MiB, the
//!   budget in cents.
//! - The facts are handed on: the envelope to `harness/`, the budget to
//!   `money/`. This type extracts; it states neither.
//! - The design doc is the team's document, so whatever it sets, the team
//!   set.
//!
//! Spec 0078.

pub mod card;
pub mod line;
pub mod read;

use crate::money::cents::Cents;
use crate::request::link::Link;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Design {
	pub link: Link,
	pub cpu_millis: i64,
	pub memory_mib: i64,
	pub budget: Cents,
}
