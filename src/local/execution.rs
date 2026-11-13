//! The execution ledger
//!
//! The execution ledger is the run's private record of what it created on the
//! machine: its space in the cluster and the Deployments' names. Each name is
//! written before the thing is created, so that cleanup removes exactly what
//! the run created. It is settled once everything is removed and the space's
//! absence is confirmed. It is not the money ledger, which is
//! `money/ledger.rs`.
//!
//! Definition only: the `Execution` type: the run and its runtime, the space,
//! the Deployments' names, the names already removed and whether the ledger
//! is settled. Its functions are the files in `execution/`.
//!
//! ## Rules
//!
//! - It lives at `.langdrift/agent/state/runs/<run>/execution.json`, beside
//!   the run's stage reports. `<run>` is the run's id, from `evidence/`.
//! - It is private.
//! - The space is named after the run and this project (`space`), so that
//!   two checkouts on one machine never share one.
//! - It feeds the removed count and "nothing left" of the stopped line.
//! - It lets a later run remove what a stopped run left.

pub mod card;
pub mod cleanup;
pub mod left;
pub mod load;
pub mod open;
pub mod placed;
pub mod record;
pub mod space;
pub mod sweep;
pub mod up;
pub mod write;

use serde::{Deserialize, Serialize};

pub const FILE: &str = "execution.json";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Execution {
	pub run: String,
	pub runtime: String,
	pub space: String,
	pub deployments: Vec<String>,
	pub removed: Vec<String>,
	pub settled: bool,
}
