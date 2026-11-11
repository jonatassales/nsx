//! A readiness, as the run's record spells it
//!
//! One function: it gives back the word `run.json` keeps for where the pull
//! request stands: `draft`, `ready for review` or `ready to merge`.
//!
//! Spec 0081.

use crate::contract::pull::{DRAFT, MERGE, REVIEW};
use crate::delivery::readiness::Readiness;

impl Readiness {
	pub fn word(&self) -> &'static str {
		match self {
			Self::Draft => DRAFT,
			Self::Review => REVIEW,
			Self::Merge => MERGE,
		}
	}
}
