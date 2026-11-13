//! The time, behind a port
//!
//! The run's record keeps moments: when the effort was recorded
//! (`recordedAt`) and when Safe Seat was decided (`decidedAt`). The clock is
//! one of the boundaries outside the project, so the golden test scripts it.
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
//! - The form of a recorded moment. Only "a timestamp" is settled: the form
//!   above holds until the owner decides.

pub trait Clock {
	fn now(&self) -> String;
}
