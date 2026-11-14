//! Whether a pair is a candidate
//!
//! One function: it takes a model's id and an effort, and says whether the
//! candidates hold that model at that effort.

use crate::money::candidates::Candidates;

impl Candidates {
	pub fn holds(&self, model: &str, effort: &str) -> bool {
		self.models.iter().any(|candidate| {
			candidate.model.id == model
				&& candidate.efforts.iter().any(|offered| offered == effort)
		})
	}
}
