//! Checking the intent before a run
//!
//! One function: it takes the project's root, the paths under `.langdrift/`,
//! the recorded intent `choose.rs` gave and the price list, and gives back
//! the authorized intent, or the cause of a stop.
//!
//! ## Reads and writes
//!
//! - reads `.langdrift/microruntimes/runtimes/<runtime id>/runtime.md`, for
//!   the draft's revision
//!
//! ## Rules
//!
//! - `status` must be `authorized`: nothing runs before the team authorizes.
//! - `revision` must be the draft's current revision: the run never applies
//!   a plan the team did not see.
//! - `model` must be in the price list and `effort` one of the efforts
//!   (specs 0056, 0057).
//! - Any failure is a stop before the run's opening comment: nothing on
//!   standard output, one sentence on standard error, exit status 1.
//! - In a project connected to the console, a run whose intent is still
//!   proposed first waits for the team's Play (spec 0076). This check comes
//!   after.
//!
//! Spec 0079.

use crate::contract::{fence::Fence, intent::Intent};
use crate::files::{
	layout::Layout,
	root::Root,
	text::{DOCUMENT, text},
};
use crate::handoff::{authorization::Authorization, error::Error};
use crate::money::price::Price;

impl Authorization {
	pub fn check(
		root: &Root,
		layout: &Layout,
		intent: Intent,
		prices: &Price,
	) -> Result<Self, Error> {
		let refused = |rule: String| Error::Unauthorized {
			ticket: intent.ticket.clone(),
			rule,
		};
		let problems = intent
			.validate()
			.iter()
			.map(|problem| format!("{} {}", problem.field, problem.rule))
			.collect::<Vec<_>>();

		if !problems.is_empty() {
			return Err(refused(problems.join("; ")));
		}

		if intent.status != "authorized" {
			return Err(refused(String::from("the team has not authorized it yet")));
		}

		let draft = layout.runtimes.join(&intent.runtime_id).join("runtime.md");
		let written = text(root, &draft, DOCUMENT)?;
		let fence = Fence::parse(&written).map_err(|cause| Error::Draft {
			cause,
		})?;
		let priced = prices.models.iter().any(|model| model.id == intent.model);

		if fence.revision != intent.revision {
			return Err(refused(format!(
				"it names revision {} and the draft is at revision {}",
				intent.revision, fence.revision
			)));
		}

		if !priced {
			return Err(refused(format!(
				"its model {} is not in the price list",
				intent.model
			)));
		}

		Ok(Self {
			intent,
		})
	}
}
