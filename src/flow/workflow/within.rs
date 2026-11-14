//! Whether the workflow's default choice is offered
//!
//! One function: it takes the paths under `.langdrift/` and the candidates,
//! and checks that the workflow's default model, at its default effort, is
//! one of them.
//!
//! ## Rules
//!
//! - A default the team does not offer on the console's Settings → AI
//!   Models stops the plan before its first line, as a default outside the
//!   price list does: the plan never proposes a choice the team cannot make.

use crate::files::layout::Layout;
use crate::flow::{error::Error, workflow::Workflow};
use crate::money::candidates::Candidates;

impl Workflow {
	pub fn within(
		&self,
		layout: &Layout,
		candidates: &Candidates,
	) -> Result<(), Error> {
		let defaults = &self.defaults;

		if candidates.holds(&defaults.model, &defaults.effort) {
			return Ok(());
		}

		Err(Error::Rule {
			path: layout.workflow.clone(),
			rule: format!(
				"its default model {} at the effort {} is not offered on the console's Settings → AI Models",
				defaults.model, defaults.effort
			),
		})
	}
}
