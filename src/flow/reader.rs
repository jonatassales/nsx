//! Where a person's answer is read
//!
//! A run asks a person twice: the owner at a checkpoint, and the person who
//! writes the dev ticket. Both answer on the run's standard input, which sits
//! behind this trait so that a test can answer in the person's place.
//!
//! Definition only: the `Reader` trait: one line of what the person entered,
//! or nothing when the input has ended.
//!
//! ## Rules
//!
//! - Nothing is printed to ask: the `waiting` line already did.
//!
//! Spec 0080.

pub trait Reader {
	fn line(&self) -> std::io::Result<Option<String>>;
}
