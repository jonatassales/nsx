//! The models and efforts a plan may suggest and price
//!
//! What the price list and the team's offer have in common: the models the
//! judge may suggest, each at the efforts it may suggest, and the pairs the
//! Workflow card prices.
//!
//! Definition only: the `Candidates` type: one `Candidate` per model, in the
//! order of the three models nsx reads. Its functions are the files in
//! `candidates/`.
//!
//! ## Rules
//!
//! - A connected plan reads the team's offer before the spend table is
//!   built. A project with no connection plans with the three models at the
//!   three efforts.
//! - A candidate is a model of the price list that any provider offers, at
//!   the efforts nsx knows that the providers offering it offer. A model
//!   offered at no effort nsx knows is no candidate.
//! - Only Anthropic's models are priced: a model of another provider is never
//!   a candidate, and never priced at zero.
//! - An offer read from the console that leaves no candidate stops the plan
//!   before its first line, naming the console's Settings → AI Models.
//!
//! ## Open
//!
//! - Whether a run checks its choice against the offer again. Until the
//!   owner decides, the run takes the pair from the spend table the plan
//!   priced, so a pair the offer left out stops it there.

pub mod holds;
pub mod of;
pub mod read;

use crate::money::candidate::Candidate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidates {
	pub models: Vec<Candidate>,
}
