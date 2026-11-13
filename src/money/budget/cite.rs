//! The budget's line in the run
//!
//! One function: it gives back the run's line that states the budget and
//! cites the criterion it satisfies.
//!
//! ## Simulation
//!
//! - `budget` (run): `console › budget     $25.00 per run · set by the team · FIN-03`
//!
//! ## Rules
//!
//! - The line cites FIN-03, a budget before a paid run. Nothing has been
//!   reserved or paid when it prints.
//! - It is the plan's budget in other words: both lines come from one type.

use crate::money::budget::Budget;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Budget {
	pub fn cite(&self) -> Line {
		Line {
			id: String::from("budget"),
			kind: Kind::Output,
			shape: Shape::Labeled {
				label: String::from("budget"),
				value: format!(
					"{} per run · set by the team · FIN-03",
					self.amount.format()
				),
			},
		}
	}
}
