//! The paths under `.langdrift/`
//!
//! Where each file of LangDrift's folder lives, so that no path is spelled in
//! two places. A runtime's folder is named after the ticket's slug, and a
//! run's files after the run's id.
//!
//! Definition only: the `Layout` type: the paths below, built from the root.
//! Its function is the file in `layout/`.
//!
//! ## Rules
//!
//! - `nsx init` lays down `finops/prices.json`, `finops/forecasts.json`,
//!   `harness/workflow.json`, `harness/guardrails/0001-finops.gr.md`,
//!   `skills/` and `templates/`. The team writes `harness/driver.json`.
//! - A runtime's folder is `microruntimes/runtimes/<runtime id>/`. It holds
//!   `runtime.md`, `plan.json`, `intent.json`, `run.json` and `usage.json`.
//! - A run's id is `<runtime id>-<NNNN>`. Its money is
//!   `finops/ledger/<run id>.ledger.json`.
//! - A run's private files are under `agent/state/runs/<run id>/`: `stages/`
//!   and `execution.json`.
//! - The kubeconfig nsx reaches the local cluster with is private too:
//!   `agent/state/cluster/kubeconfig`. The person's own is never touched.
//! - An episode is `microruntimes/episodes/ep-NNNN-<slug>.json`, and the
//!   index of episodes is `agent/state/rag/index.json`.
//! - No path the specs removed comes back: no project identity file, no
//!   input record, no mirror and no `harness/envelope.json`.
//!
//! ## Open
//!
//! - Where the console's connection is recorded. Spec 0073 says the project's
//!   private state, in a credentials file with mode 0600 that Git must not
//!   track, and names no path.
//! - Where nsx keeps the console's id of a runtime. Spec 0073 says private,
//!   git-ignored state, and names no path.
//!
//! Shared: used by every folder that reads or writes under `.langdrift/`.

pub mod under;

use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layout {
	pub langdrift: PathBuf,
	pub prices: PathBuf,
	pub forecasts: PathBuf,
	pub ledger: PathBuf,
	pub workflow: PathBuf,
	pub driver: PathBuf,
	pub guardrails: PathBuf,
	pub skills: PathBuf,
	pub templates: PathBuf,
	pub runtimes: PathBuf,
	pub episodes: PathBuf,
	pub index: PathBuf,
	pub credentials: PathBuf,
	pub ignored: PathBuf,
	pub reported: PathBuf,
	pub runs: PathBuf,
	pub kubeconfig: PathBuf,
}
