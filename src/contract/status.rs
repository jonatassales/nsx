//! Where a runtime stands
//!
//! The status a report states. nsx says it, and the console never derives it.
//!
//! Definition only: the `Status` type and its statuses: `ready`, `analysis`,
//! `plan`, `workflow`, `held`, `paused`, `done` and `error`.
//!
//! ## Rules
//!
//! - `ready`: `nsx init` connected the project, and nothing is planned.
//! - `analysis`: `nsx agent plan`, from the ticket read until the analysis
//!   ends.
//! - `plan`: `nsx agent plan` from then on, the proposed plan included.
//! - `workflow`: `nsx agent run` while the workflow plays, a person's waits
//!   included.
//! - `held`: `nsx agent run` waits for the team's Play, before its header or
//!   before a stage's work. It is the console's, never the gate's.
//! - `paused`: the run paused at the budget gate, and the process ended.
//! - `done`: the run ended ready for review or ready to merge.
//! - `error`: the plan or the run stopped on a failure.
//! - Any other status is refused.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
	Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "lowercase")]
#[schemars(inline)]
pub enum Status {
	Ready,
	Analysis,
	Plan,
	Workflow,
	Held,
	Paused,
	Done,
	Error,
}
