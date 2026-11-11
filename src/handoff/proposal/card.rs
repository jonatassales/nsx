//! The proposal on the Plan card
//!
//! One function: it puts where the proposal stands into the plan's cards:
//! the authorization, pending, and the evidence, the draft and its revision.
//!
//! ## Simulation
//!
//! - Card: Plan · the header's and PROPOSED PLAN's suffix, "pending team
//!   authorization"
//! - Card: Plan · EVIDENCE, `runtime.md draft · revision 1 · proposed to the console`
//!
//! ## Rules
//!
//! - Both arrive when the plan's last result line prints, and are never
//!   rewritten.
//! - This function writes no file: the one writer in `handoff/` does.
//!
//! Spec 0079.

use crate::contract::{
	authorization::Authorization, cards::Cards, evidence::Evidence,
};
use crate::handoff::proposal::Proposal;

impl Proposal {
	pub fn card(&self, cards: &mut Cards) {
		let plan = cards.plan.get_or_insert_default();

		plan.authorization = Some(Authorization::Pending);
		plan.evidence = Some(Evidence {
			file: Some(String::from("runtime.md")),
			state: Some(String::from("draft")),
			revision: Some(self.revision),
			proposed_to: Some(String::from("console")),
		});
	}
}
