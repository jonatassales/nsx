//! The sizing on the Plan card
//!
//! One function: it puts what the `size` line states into the sizing of the
//! plan's cards: the headroom the bound leaves and the suggested resource.
//!
//! ## Simulation
//!
//! - Card: Plan · CPU, `1,450m of 2,000m`, and Memory,
//!   `2,304 MiB of 3,072 MiB`, with bars at the share of the bound
//! - Card: Plan · HEADROOM, `550m · 768 MiB`
//! - Card: Plan · SUGGESTED RESOURCE, `5 one-replica Deployments`
//!
//! ## Rules
//!
//! - The proposed totals and the bound are already on the card: each was set
//!   when its own line printed.
//! - This function writes no file: the one writer in `handoff/` does.
//!
//! Spec 0079.

use crate::contract::{
	amount::Amount,
	cards::Cards,
	resource::{KIND, Resource},
};
use crate::runtime::sizing::Sizing;

impl Sizing {
	pub fn card(&self, cards: &mut Cards) {
		let section = cards.sizing();

		section.headroom = Some(Amount {
			cpu_millis: self.headroom_cpu_millis,
			memory_mib: self.headroom_memory_mib,
		});
		section.resource = Some(Resource {
			kind: String::from(KIND),
			count: self.deployments as u64,
			replicas: 1,
		});
	}
}
