//! The budget a design doc sets
//!
//! One function: it takes the design doc as `request/` read it and gives back
//! the budget per run, with the design doc's path as its source.

use crate::money::budget::Budget;
use crate::request::design::Design;

impl Budget {
	pub fn of(design: &Design) -> Self {
		Self {
			amount: design.budget,
			source: design.link.path.clone(),
		}
	}
}
