//! Counting what was read
//!
//! One function: it takes the ticket's links and gives back the two counts
//! the `✓ read` line and the card show.
//!
//! ## Rules
//!
//! - The card counts the documents the ticket links.
//! - The terminal counts what was read before its line: the ticket itself and
//!   every linked document but the guardrail, which `harness/` reads next. That
//!   is as many as the links.

use crate::request::{links::Links, reading::Reading};

impl Reading {
	pub fn count(links: &Links) -> Self {
		let linked =
			[&links.prd, &links.design, &links.spec, &links.guardrail].len();

		Self {
			read: linked as u64,
			linked: linked as u64,
		}
	}
}
