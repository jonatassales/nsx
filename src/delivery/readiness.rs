//! Where a pull request stands
//!
//! Definition only: the `Readiness` type: a pull request is a draft until
//! the run marks it ready, and then it is ready for review or ready to
//! merge. Its function is the file in `readiness/`.
//!
//! ## Rules
//!
//! - Which of the two a ready pull request is follows the Review tick and
//!   nothing else.

pub mod word;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Readiness {
	Draft,
	Review,
	Merge,
}
