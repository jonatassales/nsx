//! Recording the effort
//!
//! One function: it takes the project's root, the console, the applied
//! intent, the judge's suggested pair as the plan's cards carry it, the
//! clock and the run's record. It writes the effort record into the run's
//! record and gives it back, which lets the implement call start
//! (`permit.rs`).
//!
//! ## Reads and writes
//!
//! - writes the `effort` section of `run.json`, through `evidence/record.rs`,
//!   the only writer of that file
//!
//! ## Rules
//!
//! - It runs once per run, at the start of Implement: after Implement's
//!   confirmation when Implement is ticked, and before the implement call.
//! - It runs whether or not Implement is ticked.
//! - If the record cannot be written, the stop rule applies: the run stops
//!   before the `hook` line, implement is never called and nothing is
//!   printed instead.
//!
//! Spec 0080.

use crate::console::client::Client;
use crate::contract::{effort::Effort as Recorded, intent::Intent};
use crate::evidence::{clock::Clock, error::Error, record::Record};
use crate::files::root::Root;
use crate::hook::effort::Effort;

impl Effort {
	pub async fn record(
		root: &Root,
		console: &dyn Client,
		intent: &Intent,
		suggested: (Option<String>, Option<String>),
		clock: &dyn Clock,
		record: &mut Record,
	) -> Result<Self, Error> {
		let (suggested_model, suggested_effort) = suggested;
		let effort = Self {
			model: intent.model.clone(),
			effort: intent.effort.clone(),
			suggested_model,
			suggested_effort,
			recorded_at: clock.now(),
		};

		record
			.write(root, console, |run| {
				run.effort = Some(Recorded {
					applied_model: effort.model.clone(),
					applied_effort: effort.effort.clone(),
					suggested_model: effort.suggested_model.clone(),
					suggested_effort: effort.suggested_effort.clone(),
					recorded_at: effort.recorded_at.clone(),
				});
			})
			.await?;

		Ok(effort)
	}
}
