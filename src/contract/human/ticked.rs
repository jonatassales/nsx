//! Whether a stage has a person in it
//!
//! One function: it takes a stage's id and says whether the choice ticks it.
//!
//! ## Rules
//!
//! - Planning is always ticked.
//! - Spec, Tickets, Implement and Review are ticked as the choice says.
//! - Any other stage has no box: the dev ticket is always the person's and
//!   Merge always the owner's, and neither is a tick.
//!
//! Spec 0077.

use crate::contract::human::Human;

impl Human {
	pub fn ticked(&self, stage: &str) -> bool {
		match stage {
			"planning" => true,
			"spec" => self.spec,
			"tickets" => self.tickets,
			"implement" => self.implement,
			"review" => self.review,
			_ => false,
		}
	}
}
