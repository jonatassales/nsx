//! The budget per run
//!
//! What one run may spend on model calls. The team sets it in the design doc:
//! $25.00 per run for the simulation's ticket. The gate compares the
//! reservation with it, and it is the only bound on a run's money.
//!
//! Definition only: the `Budget` type: the amount in cents and its source, the
//! design doc's path. Its functions are the files in `budget/`.
//!
//! ## Rules
//!
//! - `request/` reads the budget out of the design doc, before the plan's
//!   first line. This type states it and does not extract it.
//! - It is always "set by the team": there is no owner input, no budgets file,
//!   no allocation per ticket and no period envelope.
//! - The run reads the budget the plan recorded, so the gate is the one the
//!   card described.
//! - The console's team budget (spec 0075) is a different thing: nsx neither
//!   sets, reads nor sends it.
//!
//! ## Open
//!
//! - Raising the budget after a pause means editing the design doc. Whether
//!   that needs a new plan, or the next `nsx agent run` reads it again, is
//!   open.
//!
//! Spec 0079.

pub mod cite;
pub mod line;
pub mod of;

use std::path::PathBuf;

use crate::money::cents::Cents;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Budget {
	pub amount: Cents,
	pub source: PathBuf,
}
