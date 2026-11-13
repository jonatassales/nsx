//! Whether the checks passed
//!
//! One function: it says whether the suite lists at least one check and
//! every listed check passed.
//!
//! ## Rules
//!
//! - A suite with no check has not passed: nothing was checked.

use crate::checks::{standing::Standing, suite::Suite};

impl Suite {
	pub fn passed(&self) -> bool {
		let all = self
			.checks
			.iter()
			.all(|check| check.standing == Standing::Passed);

		!self.checks.is_empty() && all
	}
}
