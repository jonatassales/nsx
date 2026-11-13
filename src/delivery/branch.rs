//! The run's branch
//!
//! The branch the stages and the person work on, so that the delivery never
//! lands on the branch the developer was on. It is named after the ticket:
//! `feat/<ticket number>-<ticket slug>`, which is
//! `feat/0031-book-a-demo-runtime` for the simulation's ticket.
//!
//! Definition only: the `Branch` type, which carries the branch's name and,
//! once pushed, the pushed head's hash, whole. Its functions are the files
//! in `branch/`.
//!
//! ## Rules
//!
//! - The number and the slug are the ticket's.
//! - The run's record keeps `branch` and `commit`, the hash's first seven
//!   characters.
//!
//! ## Open
//!
//! - A second run of the same ticket. Until the owner decides, it reuses the
//!   branch an earlier run left. The alternatives are a fresh branch name per
//!   run, or a stop.

pub mod card;
pub mod line;
pub mod push;
pub mod short;
pub mod start;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Branch {
	pub name: String,
	pub commit: Option<String>,
}
