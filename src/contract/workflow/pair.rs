//! Looking a pair up in the spend table
//!
//! One function: it takes a model and an effort and gives back their row of
//! the spend table, if the table holds one.
//!
//! ## Rules
//!
//! - Every money figure of a run is this lookup: nothing is priced again.
//!
//! Spec 0077.

use crate::contract::{pair::Pair, workflow::Workflow};

impl Workflow {
	pub fn pair(&self, model: &str, effort: &str) -> Option<&Pair> {
		self
			.spend
			.iter()
			.find(|pair| pair.model == model && pair.effort == effort)
	}
}
