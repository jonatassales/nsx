//! The budget's line in the plan
//!
//! One function: it gives back the plan's line that states the budget and who
//! set it. The run states it in other words, in `cite.rs`.
//!
//! ## Simulation
//!
//! - `budget` (plan): `budget     $25.00 per run for model calls · set by the team in the design doc`
//!
//! ## Rules
//!
//! - The line follows the `bounded` line and changes no card field: the
//!   budget is already on the design doc's row.

use crate::money::budget::Budget;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Budget {
	pub fn line(&self) -> Line {
		Line {
			id: String::from("budget"),
			kind: Kind::Output,
			shape: Shape::Status {
				label: String::from("budget"),
				value: format!(
					"{} per run for model calls · set by the team in the design doc",
					self.amount.format()
				),
			},
		}
	}
}
