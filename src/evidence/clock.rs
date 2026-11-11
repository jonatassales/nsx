//! The time, behind a port
//!
//! The run's record keeps moments: when the effort was recorded
//! (`recordedAt`, spec 0060) and when Safe Seat was decided (`decidedAt`,
//! spec 0061). The clock is one of the boundaries outside the project, so
//! the golden test scripts it.
//!
//! Definition only: the trait `Clock`, which gives the current moment as
//! text, so that tests can replace it with a fake.
//!
//! ## Rules
//!
//! - A moment is an RFC 3339 timestamp in UTC, such as
//!   `2026-11-11T14:03:27Z`.
//!
//! ## Open
//!
//! - The form of a recorded moment. The specs say "a timestamp" and no more:
//!   the form above holds until the owner decides.
//!
//! Spec 0080.

pub trait Clock {
	fn now(&self) -> String;
}
