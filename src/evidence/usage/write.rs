//! Keeping the usage record
//!
//! One function: it takes the project's root, the usage file and how one
//! stage-driver call ended, adds the call to this run's entry in
//! `usage.json` and sums the run's totals. It gives back nothing, or the
//! failure to write.
//!
//! ## Reads and writes
//!
//! - reads and writes
//!   `.langdrift/microruntimes/runtimes/<runtime id>/usage.json`, a list of
//!   the runtime's runs
//!
//! ## Rules
//!
//! - One entry per run that made at least one paid call, appended. It
//!   matches the money ledger's entry for the same run.
//! - A call that reported no tokens is kept as reported, and the totals
//!   become `null`. So does a total too large to count.
//! - A stop keeps every paid call already in the record: no spend goes
//!   unrecorded.
//!
//! Spec 0080.

use std::path::Path;

use crate::contract::encode::encode;
use crate::driver::ended::Ended;
use crate::evidence::{error::Error, usage::Usage, used::Used};
use crate::files::{inside::inside, root::Root, write::write};

impl Usage {
	pub fn write(
		&mut self,
		root: &Root,
		file: &Path,
		ended: &Ended,
	) -> Result<(), Error> {
		let target = inside(root, file)?;
		// The sum of one count over the calls: unknown when any is unknown.
		let total = |calls: &[Used], count: fn(&Used) -> Option<u64>| {
			calls
				.iter()
				.map(count)
				.try_fold(0_u64, |sum, tokens| sum.checked_add(tokens?))
		};

		self.calls.push(Used {
			stage: ended.stage.clone(),
			skill: ended.skill.clone(),
			input_tokens: ended.tokens.input,
			output_tokens: ended.tokens.output,
		});
		self.input_tokens = total(&self.calls, |used| used.input_tokens);
		self.output_tokens = total(&self.calls, |used| used.output_tokens);

		let mut runs = Self::kept(root, file)?;

		runs.retain(|entry| entry.run != self.run);
		runs.push(self.clone());

		let mut written = encode(&runs).map_err(|cause| Error::Unwritten {
			path: target.clone(),
			cause,
		})?;

		written.push('\n');

		Ok(write(&target, written.as_bytes())?)
	}
}
