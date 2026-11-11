//! The tokens a call used
//!
//! The input and output tokens of one stage call, as the coding agent's own
//! output reports them. They feed the usage record (spec 0065), from which
//! the observed spend is computed (spec 0056): measured, not guessed.
//!
//! Definition only: the `Tokens` type, which carries a call's input tokens
//! and its output tokens. Its functions are the files in `tokens/`.
//!
//! ## Rules
//!
//! - Tokens that were not reported are unknown (`null`), never zero.
//! - The driver reads no cost: the observed spend is the list price applied
//!   to the tokens (spec 0056).
//!
//! Spec 0080.

pub mod read;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tokens {
	pub input: Option<u64>,
	pub output: Option<u64>,
}
