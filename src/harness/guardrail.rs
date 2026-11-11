//! The guardrail
//!
//! A document of criteria that the team keeps in the harness and a ticket
//! links, such as the FinOps guardrail. The plan reads it and records it as a
//! review rule with no automatic enforcement.
//!
//! Definition only: the `Guardrail` type: its project-relative path, its file
//! name, its short id and its criteria in table order. Each criterion says
//! whether the plan step checks it. The enforcement is always review, so it
//! is not kept. Its functions are the files in `guardrail/`.
//!
//! ## Simulation
//!
//! - Card: Analysis · the FIN line under the guardrail row, set when the
//!   judge's line on the criteria prints
//!
//! ## Rules
//!
//! - nsx never enforces a criterion. The money gates are their own mechanism
//!   and cite FIN ids only as references.
//! - The FIN line shows each criterion as `<id>: <short statement>`, joined by
//!   ` · `, in table order. The statements are the judge's.
//!
//! Spec 0078.

pub mod card;
pub mod line;
pub mod load;

use std::path::PathBuf;

use crate::harness::criterion::Criterion;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Guardrail {
	pub path: PathBuf,
	pub name: String,
	pub short: String,
	pub criteria: Vec<Criterion>,
}
