//! The verdict in the run's record
//!
//! One function: it puts the verdict into the Safe Seat section of the
//! run's record, when Safe Seat was met.
//!
//! ## Simulation
//!
//! - Card: Workflow · SAFE SEAT: a dash until `✓ Safe Seat  met` has printed,
//!   then `met`, in the success tone
//!
//! ## Rules
//!
//! - A verdict that is not met writes nothing: the section stays unset.
//! - This function writes no file: `evidence/record/write.rs` does.

use crate::contract::{
	run::Run,
	seat::{MET, Seat},
};
use crate::seat::verdict::Verdict;

impl Verdict {
	pub fn card(&self, run: &mut Run) {
		if !self.met {
			return;
		}

		run.safe_seat = Some(Seat {
			safe_seat: String::from(MET),
			decided_at: self.decided_at.clone(),
		});
	}
}
