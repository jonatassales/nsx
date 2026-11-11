//! The intent a proposal records
//!
//! One function: it gives back the intent the proposal stands for, as
//! `intent.json` holds it and as the console is told it.
//!
//! ## Rules
//!
//! - Its status is `proposed`.
//! - Planning is always a human stage, whatever the workflow's default says.
//!
//! Spec 0079.

use crate::contract::{human::Human, intent::Intent};
use crate::handoff::proposal::Proposal;

impl Proposal {
	pub fn intent(&self) -> Intent {
		Intent {
			ticket: self.ticket.clone(),
			runtime_id: self.runtime.clone(),
			revision: self.revision,
			model: self.choice.model.clone(),
			effort: self.choice.effort.clone(),
			human: Human {
				planning: true,
				..self.choice.human.clone()
			},
			status: String::from("proposed"),
		}
	}
}
