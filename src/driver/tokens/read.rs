//! Reading a call's tokens
//!
//! One function: it takes what the coding agent printed and the two usage
//! paths of the driver configuration, and gives back the call's input and
//! output tokens.
//!
//! ## Rules
//!
//! - The paths are `usage.inputTokens` and `usage.outputTokens` of
//!   `driver.json`: dotted paths into the agent's JSON output.
//! - The output is one JSON object, or text whose last line is one.
//! - A count is a whole number, never negative. A path that leads nowhere,
//!   or to anything else, leaves that count unknown: unknown is never zero.
//! - Output that does not yield both counts at those paths makes the call a
//!   failed call, with its tokens unknown.

use serde_json::Value;

use crate::driver::{paths::Paths, tokens::Tokens};

impl Tokens {
	pub fn read(printed: &str, paths: &Paths) -> Self {
		// The whole output is the report, or else its last line is.
		let last = printed.lines().rev().find(|line| !line.trim().is_empty());
		let output = serde_json::from_str::<Value>(printed)
			.ok()
			.or_else(|| last.and_then(|line| serde_json::from_str(line).ok()));
		let count = |path: &str| {
			let mut found = output.as_ref()?;

			for key in path.split('.') {
				found = found.get(key)?;
			}

			found.as_u64()
		};

		Self {
			input: count(&paths.input_tokens),
			output: count(&paths.output_tokens),
		}
	}
}
