//! The run's id
//!
//! Every `nsx agent run` gets one id when it starts, before its first line:
//! the runtime id and a four-digit sequence, as in
//! `book-a-demo-runtime-0001`. The money ledger's file, the run's private
//! folder and the stage driver are named by it.
//!
//! Definition only: the `Id` type, which carries the runtime id and the
//! run's number. Its functions are the files in `id/`.
//!
//! ## Rules
//!
//! - It prints nowhere. It is kept in `run.json` as `run`.
//! - It names `finops/ledger/<run>.ledger.json` (spec 0056), the private
//!   folder `agent/state/runs/<run>/` (specs 0058, 0059, 0062), and the
//!   stage driver's `{{run}}` and `LANGDRIFT_RUN` (spec 0058).
//!
//! ## Open
//!
//! - The id's form. The simulation shows no run id, and the spec chooses
//!   `<runtime id>-<NNNN>`. The owner may prefer another identity, such as a
//!   timestamp or one tied to the intent's revision.
//!
//! Spec 0080.

pub mod mint;
pub mod text;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Id {
	pub runtime: String,
	pub number: u32,
}
