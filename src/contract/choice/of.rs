//! The choice an intent holds
//!
//! One function: it takes an intent and gives back the choice a Play of it
//! would carry: its revision, model, effort and checkpoints.
//!
//! Spec 0077.

use crate::contract::{choice::Choice, intent::Intent};

impl Choice {
	pub fn of(intent: &Intent) -> Self {
		Self {
			revision: intent.revision,
			model: intent.model.clone(),
			effort: intent.effort.clone(),
			human: intent.human.clone(),
		}
	}
}
