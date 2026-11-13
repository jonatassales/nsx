//! What a program printed and how it ended
//!
//! The result of running a program: enough for the caller to read an answer
//! from what was printed and to tell success from failure.
//!
//! Definition only: the `Output` type: what the program printed on standard
//! output, what it complained of on standard error, and its exit status,
//! which is unknown when a signal ended it. Its function is the file in
//! `output/`.
//!
//! ## Rules
//!
//! - It is never printed as it is. The coding agent's output stays in a
//!   private log and never enters evidence.
//!
//! ## Open
//!
//! - Whether a call's elapsed time is measured, by this type or by `driver`,
//!   is not settled.
//! - Whether what a program printed is kept whole or only up to a limit.
//!
//! Shared: used by `driver`, `local` and `delivery`.

pub mod ended;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Output {
	pub status: Option<i32>,
	pub printed: Vec<u8>,
	pub complained: Vec<u8>,
}
