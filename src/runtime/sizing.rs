//! The sizing of the plan
//!
//! The plan proposes the sum of the components' own shares, never an equal
//! split of the bound. The sum must fit the bound the team set in the design
//! doc, and the headroom is what the bound leaves. The plan proposes one
//! one-replica Deployment per component.
//!
//! Definition only: the `Sizing` type: the proposed totals, the bound and the
//! headroom, each in cpu and memory, and the number of one-replica
//! Deployments. Its functions are the files in `sizing/`.
//!
//! ## Simulation
//!
//! - Card: Plan · BOUNDED, `2,000m · 3,072 MiB`, set from the `bounds` line of
//!   spec 0054
//!
//! ## Rules
//!
//! - Amounts are whole millicores and whole MiB.
//! - Headroom is never clamped at zero: a sum over the bound stops the plan.
//! - What the run applies is exactly these shares: the runtime's ready line
//!   reads `1450m · 2304 MiB` from them (spec 0062).
//!
//! Spec 0079.

pub mod card;
pub mod fit;
pub mod line;
pub mod proposed;
pub mod sum;

use crate::harness::envelope::Envelope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sizing {
	pub cpu_millis: i64,
	pub memory_mib: i64,
	pub bound: Envelope,
	pub headroom_cpu_millis: i64,
	pub headroom_memory_mib: i64,
	pub deployments: usize,
}
