//! The proposed totals on the Plan card
//!
//! One function: it puts the totals of the components resolved so far into
//! the sizing of the plan's cards.
//!
//! ## Simulation
//!
//! - Card: Plan · header figures and PROPOSED PLAN figures, stepping up at
//!   each component's row: `250m · 256 MiB`, `550m · 768 MiB`,
//!   `750m · 1,024 MiB`, `950m · 1,280 MiB`, then `1,450m · 2,304 MiB`
//! - Card: Plan · components' total line, `1 of 5 resolved · 250m · 256 MiB`
//!   while components resolve, then `5 components · 1,450m · 2,304 MiB`
//!
//! ## Rules
//!
//! - The figures are not set before the first component resolves.
//! - They are the sum as it stands, even over the bound: the plan says so
//!   before its `size` line, and stops there.
//! - How many components are resolved is the number of filled rows of the
//!   card's COMPONENTS. This function adds the totals.
//! - This function writes no file: the one writer in `handoff/` does.
//!
//! Spec 0079.

use crate::contract::{amount::Amount, cards::Cards};
use crate::runtime::sizing::Sizing;

impl Sizing {
	pub fn proposed(&self, cards: &mut Cards) {
		cards.sizing().proposed = Some(Amount {
			cpu_millis: self.cpu_millis,
			memory_mib: self.memory_mib,
		});
	}
}
