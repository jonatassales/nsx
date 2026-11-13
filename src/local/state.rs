//! The runtime, ready or stopped
//!
//! The runtime is ready once every Deployment is available. It is stopped
//! once the Deployments and the run's space are gone and the space's absence
//! is confirmed. The run's record keeps this state, so that the evidence
//! agrees with the two runtime lines.
//!
//! Definition only: the `State` type: the runtime's id, the revision, the
//! number of Deployments, the applied totals in millicores and MiB and, once
//! stopped, the number removed. Its functions are the files in `state/`.
//!
//! ## Rules
//!
//! - The state goes into the runtime section of `run.json`, through the
//!   record's one writer, in `evidence/`. This file writes nothing itself. The
//!   settled execution ledger marks that section stopped
//!   (`execution/card.rs`), on a silent take-down too.
//! - The fields are `revision`, `deployments` (`5`), the applied totals
//!   `cpuMillis` (`1450`) and `memoryMiB` (`2304`), `state` (`ready`, then
//!   `stopped`), `removed` (`5`) and `left` (`0`).
//! - The record is token-free.

pub mod card;
pub mod line;
pub mod of;
pub mod stopped;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
	pub runtime: String,
	pub revision: i64,
	pub deployments: u64,
	pub cpu_millis: i64,
	pub memory_mib: i64,
	pub removed: Option<u64>,
}
