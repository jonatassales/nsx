//! Writing the draft
//!
//! One function: it takes the project's root, the runtime's file, a draft
//! and the count its validation gave, 0, and writes the draft as the
//! runtime's file, with its revision. The hand-off then gets the draft's
//! path and revision.
//!
//! ## Reads and writes
//!
//! - writes `.langdrift/microruntimes/runtimes/<runtime id>/runtime.md`
//!
//! ## Rules
//!
//! - `<runtime id>` is the ticket's slug (spec 0053): `book-a-demo-runtime`
//!   for the simulation's ticket.
//! - The draft is written only when its validation count is 0, after the
//!   `validate` line and before the `draft` line of spec 0050.
//! - A plan that stops writes no draft.
//! - The file is a title, the runtime's id; the `langdrift` block, the fence
//!   as JSON; and `## Sizing`, the report: a table of the proposed totals,
//!   the bound and the headroom, then one line with the resource, the limits
//!   statement and the validation result.
//! - The file is written whole or not at all, over an earlier draft.
//!
//! Spec 0079.

use std::path::Path;

use crate::contract::{
	encode::encode,
	fence::{CLOSING, OPENING},
};
use crate::files::{inside::inside, root::Root, write::write};
use crate::runtime::{draft::Draft, error::Error};

impl Draft {
	pub fn write(
		&self,
		root: &Root,
		file: &Path,
		errors: usize,
	) -> Result<(), Error> {
		let inside = inside(root, file)?;
		let fence = encode(&self.fence).map_err(|cause| Error::Unwritten {
			path: inside.clone(),
			cause,
		})?;
		let sizing = &self.sizing;
		let report = [
			String::from("| | cpu | memory |"),
			String::from("| --- | --- | --- |"),
			format!(
				"| Proposed | {}m | {} MiB |",
				sizing.cpu_millis, sizing.memory_mib
			),
			format!(
				"| Bound, set by the team in the design doc | {}m | {} MiB |",
				sizing.bound.cpu_millis, sizing.bound.memory_mib
			),
			format!(
				"| Headroom | {}m | {} MiB |",
				sizing.headroom_cpu_millis, sizing.headroom_memory_mib
			),
		];
		let text = format!(
			"# {}\n\n{OPENING}{fence}{CLOSING}\n\n## Sizing\n\n{}\n\n{} Deployments, 1 replica each. gpu and disk are not runtime.md fields; none requested. Schema validation: {errors} errors.\n",
			self.fence.runtime_id,
			report.join("\n"),
			sizing.deployments
		);

		Ok(write(&inside, text.as_bytes())?)
	}
}
