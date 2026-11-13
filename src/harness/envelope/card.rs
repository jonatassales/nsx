//! The bound on the Plan card
//!
//! One function: it puts the bound into the sizing of the plan's cards, with
//! the design doc's path as its source.
//!
//! ## Simulation
//!
//! - Card: Plan · BOUNDED, `2,000m · 3,072 MiB`, set from the `bounds` line
//!
//! ## Rules
//!
//! - This function writes no file: the one writer in `handoff/` does.

use crate::contract::{bound::Bound, cards::Cards};
use crate::harness::envelope::Envelope;

impl Envelope {
	pub fn card(&self, cards: &mut Cards) {
		cards.sizing().bound = Some(Bound {
			cpu_millis: self.cpu_millis,
			memory_mib: self.memory_mib,
			source: self.source.to_string_lossy().into_owned(),
		});
	}
}
