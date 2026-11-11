//! Where a check stands
//!
//! Definition only: the `Standing` type: a check is running until it
//! completes, and then it passed or it did not.
//!
//! ## Rules
//!
//! - A check that was cancelled or skipped did not pass: it is `Failed`.
//!
//! Spec 0081.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Standing {
	Running,
	Passed,
	Failed,
}
