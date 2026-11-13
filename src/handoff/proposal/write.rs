//! Recording the proposed intent
//!
//! One function: it takes the project's root and the intent's file, and
//! writes the proposal to `intent.json`, beside the draft. It gives back
//! nothing, or the failure to write.
//!
//! ## Reads and writes
//!
//! - writes `.langdrift/microruntimes/runtimes/<runtime id>/intent.json`
//!
//! ## Rules
//!
//! - It writes once, after the draft is written and before the plan's last
//!   result line prints.
//! - A plan that stops writes no intent.
//! - The intent is checked with the contract's own rules before it is
//!   written: one that breaks a rule is refused, with each field and rule.

use std::path::Path;

use crate::contract::encode::encode;
use crate::files::{inside::inside, root::Root, write::write};
use crate::handoff::{error::Error, proposal::Proposal};

impl Proposal {
	pub fn write(&self, root: &Root, file: &Path) -> Result<(), Error> {
		let target = inside(root, file)?;
		let intent = self.intent();
		let problems = intent
			.validate()
			.iter()
			.map(|problem| format!("{} {}", problem.field, problem.rule))
			.collect::<Vec<_>>();

		if !problems.is_empty() {
			return Err(Error::Invalid {
				path: target,
				problems: problems.join("; "),
			});
		}

		let mut text = encode(&intent).map_err(|cause| Error::Unwritten {
			path: target.clone(),
			cause,
		})?;

		text.push('\n');

		Ok(write(&target, text.as_bytes())?)
	}
}
