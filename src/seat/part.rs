//! The part a person writes
//!
//! The dev-ticket stage of a run. Implement stops at the reserved part, and
//! the run hands that part to a person and waits. When the person is done,
//! the run closes the stage and states how the part was written: without an
//! agent, with no paid call.
//!
//! Definition only: the `Part` type, which carries the stage's `state`
//! (`pending`, `active` or `done`) and, once the person closed it, `closedBy`
//! (`person`) and `paidCalls`, the stage calls the run made meanwhile. The
//! role that closes it sits beside the type. Its functions are the files in
//! `part/`.
//!
//! ## Simulation
//!
//! - Card: Workflow · HUMAN IN THE LOOP, the Dev ticket row. It has no box
//!   and reads `driven by the person` throughout. While the stage is active
//!   it reads `the person writes the reserved part`. After `closed` it has a
//!   done mark.
//!
//! ## Rules
//!
//! - The stage starts only after implement's report says it stopped at the
//!   reserved part (spec 0059).
//! - `paidCalls` is a known zero: the run made no stage-driver call between
//!   the stage's two lines.
//! - The state goes into the run's record (spec 0065), within spec 0057's
//!   stage states.
//! - The run states what it did. It does not observe the person's own tools.
//!
//! ## Open
//!
//! - Who writes the dev ticket: whether it must be a particular person, and
//!   whether nsx should record who. nsx records `person`.
//!
//! Spec 0080.

pub mod close;
pub mod line;
pub mod of;
pub mod wait;

pub const PERSON: &str = "person";

pub const STAGE: &str = "dev-ticket";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Part {
	pub state: String,
	pub closed_by: Option<String>,
	pub paid_calls: Option<u64>,
}
