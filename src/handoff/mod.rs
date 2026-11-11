//! The hand-off between the plan, the team and the run
//!
//! The plan hands the team its card data and a proposed intent. The team
//! authorizes the intent, and the run applies only an authorized one. The
//! hand-off is records in the runtime's folder under `.langdrift/`, which a
//! console only receives.
//!
//! - `Writer`: the one writer of `plan.json`, the plan's card data
//! - `Proposal`: the proposed intent, written to `intent.json`
//! - `Authorization`: the intent the team authorized, checked before a run
//! - `Error`: what the hand-off can refuse
//!
//! ## Reads and writes
//!
//! - writes `.langdrift/microruntimes/runtimes/<runtime id>/plan.json`
//! - writes `.langdrift/microruntimes/runtimes/<runtime id>/intent.json`
//! - reads `.langdrift/microruntimes/runtimes/<runtime id>/intent.json`
//!
//! ## Rules
//!
//! - nsx records the team's authorization, it never decides it.
//! - This folder computes no card value: each field's value belongs to the
//!   folder that owns it.
//! - The run's record, `run.json`, is the third part of the hand-off.
//!   `evidence` keeps it (spec 0065).
//!
//! Spec 0079.

pub mod authorization;
pub mod error;
pub mod proposal;
pub mod writer;
