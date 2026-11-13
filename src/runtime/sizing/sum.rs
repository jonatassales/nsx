//! Summing the shares
//!
//! One function: it takes the components resolved so far and the bound, and
//! gives back the sizing: the totals, the headroom and the number of
//! Deployments.
//!
//! ## Simulation
//!
//! - Card: Plan · header figures and PROPOSED PLAN figures, stepping up at
//!   each component's row: `250m · 256 MiB`, `550m · 768 MiB`,
//!   `750m · 1,024 MiB`, `950m · 1,280 MiB`, then `1,450m · 2,304 MiB`
//! - Card: Plan · CPU, `1,450m of 2,000m`, and Memory,
//!   `2,304 MiB of 3,072 MiB`, with bars at the share of the bound
//! - Card: Plan · components' total line, `1 of 5 resolved · 250m · 256 MiB`
//!   while components resolve, then `5 components · 1,450m · 2,304 MiB`
//!
//! ## Rules
//!
//! - The totals are the sum of the components' shares. The headroom is the
//!   bound minus that sum, for cpu and for memory.
//! - The plan sums at each component's row, for the card's running totals,
//!   and once more before the `size` line. Whether the sum fits the bound is
//!   `fit.rs`'s to say, before that line: summing never stops the plan.
//! - The headroom is not clamped: over the bound it is negative.
//! - One one-replica Deployment is proposed per component.
//! - The figures are not set before the first component resolves.
//! - Records hold plain integers. The grouping (`1,450m`) is the card's.

use crate::components::component::Component;
use crate::harness::envelope::Envelope;
use crate::runtime::sizing::Sizing;

impl Sizing {
	pub fn sum(components: &[Component], bound: &Envelope) -> Self {
		let cpu_millis = components.iter().fold(0_i64, |sum, component| {
			sum.saturating_add(component.cpu_millis)
		});
		let memory_mib = components.iter().fold(0_i64, |sum, component| {
			sum.saturating_add(component.memory_mib)
		});

		Self {
			cpu_millis,
			memory_mib,
			bound: bound.clone(),
			headroom_cpu_millis: bound.cpu_millis.saturating_sub(cpu_millis),
			headroom_memory_mib: bound.memory_mib.saturating_sub(memory_mib),
			deployments: components.len(),
		}
	}
}
