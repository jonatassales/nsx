//! The authorized intent
//!
//! The run applies an intent the team recorded and authorized, not one it
//! decided itself. This is that intent as the run reads it from
//! `intent.json`, with the choices the team may have changed before
//! authorizing.
//!
//! Definition only: the `Authorization` type, which carries the applied
//! intent as the contract shapes it: its ticket, its runtime, its
//! `runtime.md` revision, its model, its effort and its checkpoints. Its
//! functions are the files in `authorization/`.
//!
//! ## Reads and writes
//!
//! - reads `.langdrift/microruntimes/runtimes/<runtime id>/intent.json`
//!
//! ## Rules
//!
//! - nsx records the team's authorization, it never decides it.
//! - `choose.rs` says which recorded intent the run applies: the team's
//!   authorization chooses, and the command takes no argument.
//! - The applied intent is handed on: the choice to `money`, `flow` and
//!   `driver`, the revision to `local`, and the intent to `evidence`.

pub mod check;
pub mod choose;
pub mod grant;
pub mod line;
pub mod scope;

use crate::contract::intent::Intent;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Authorization {
	pub intent: Intent,
}
