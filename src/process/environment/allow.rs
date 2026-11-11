//! Letting named variables through
//!
//! One function: it takes the names a caller allows and gives back the
//! environment a program will see: the base, plus those variables as nsx's own
//! environment holds them.
//!
//! ## Reads and writes
//!
//! - environment: `PATH`, `HOME`, `TMPDIR`, the locale, and each variable the
//!   caller names
//!
//! ## Rules
//!
//! - Only a named variable passes (ADR 0002). For the coding agent, the names
//!   are the `passEnv` of `driver.json` (spec 0058).
//! - This is the one place that reads nsx's environment on behalf of a
//!   program it starts.
//!
//! ## Open
//!
//! - What a named variable that is not set does: no spec says whether it is
//!   left out or refused. Until one does, it is left out.
//!
//! Shared: used by `driver`, `local` and `delivery`.

use crate::process::environment::Environment;

const BASE: [&str; 7] = [
	"PATH",
	"HOME",
	"TMPDIR",
	"LANG",
	"LC_ALL",
	"LC_CTYPE",
	"LC_MESSAGES",
];

impl Environment {
	pub fn allow(names: &[&str]) -> Self {
		let variables = BASE
			.iter()
			.chain(names)
			.filter_map(|name| {
				let value = std::env::var(name).ok()?;

				Some((name.to_string(), value))
			})
			.collect();

		Self {
			variables,
		}
	}
}
