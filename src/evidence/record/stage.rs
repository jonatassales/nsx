//! Writing where a stage stands
//!
//! One function: it takes the project's root, a stage's id and what changes
//! in that stage's entry, and replaces `run.json` with the record so far.
//!
//! ## Reads and writes
//!
//! - writes `.langdrift/microruntimes/runtimes/<runtime id>/run.json`
//!
//! ## Rules
//!
//! - Only the entry of that stage changes. A stage the record does not list
//!   changes nothing.
//!
//! Spec 0080.

use crate::console::client::Client;
use crate::contract::step::Step;
use crate::evidence::{error::Error, record::Record};
use crate::files::root::Root;

impl Record {
	pub async fn stage(
		&mut self,
		root: &Root,
		console: &dyn Client,
		stage: &str,
		change: impl Fn(&mut Step),
	) -> Result<(), Error> {
		self
			.write(root, console, |run| {
				let steps = run.stages.iter_mut().flatten();

				for step in steps.filter(|step| step.id == stage) {
					change(step);
				}
			})
			.await
	}
}
