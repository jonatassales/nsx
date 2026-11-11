//! The expected tokens of one effort
//!
//! How many input and output tokens a run is expected to use at one effort.
//!
//! Definition only: the `Tokens` type: the expected input and output tokens,
//! each a whole number of thousands of tokens.
//!
//! ## Rules
//!
//! - A count is a whole number, never negative and never a fraction.
//!
//! Spec 0079.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tokens {
	pub input: u32,
	pub output: u32,
}
