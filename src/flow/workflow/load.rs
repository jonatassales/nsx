//! Loading the workflow
//!
//! One function: it takes the project's root and the paths under
//! `.langdrift/`, reads the project's workflow file and gives back the
//! workflow, with its stages and its default choice.
//!
//! ## Reads and writes
//!
//! - reads `.langdrift/harness/workflow.json`
//!
//! ## Rules
//!
//! - `nsx agent plan <ticket>` loads it before its first line, so that the
//!   Workflow card shows the stages and the default choice from the start.
//! - The file lists the seven stages, in order, each with the skill nsx
//!   plays it with: until the owner decides whether a team may change them,
//!   nsx plays these seven. Its default effort is low, medium or high, and
//!   its default model is one of the price list's.
//! - A missing or unreadable file, a field the workflow does not define, or
//!   a file that breaks a rule above stops the command before its first
//!   line (the stop rule): the plan never reaches its draft with a choice
//!   it could not propose.
//!
//! ## Open
//!
//! - What a missing or malformed file does. The simulation shows no such
//!   failure: the stop above holds until the owner decides.

use crate::contract::{decode::decode, fence::EFFORTS};
use crate::files::{
	layout::Layout,
	root::Root,
	text::{DOCUMENT, text},
};
use crate::flow::{
	error::Error,
	workflow::{SKILLS, STAGES, Workflow},
};
use crate::money::price::Price;

impl Workflow {
	pub fn load(
		root: &Root,
		layout: &Layout,
		prices: &Price,
	) -> Result<Self, Error> {
		let written = text(root, &layout.workflow, DOCUMENT)?;
		let workflow: Self = decode(&written).map_err(|cause| Error::Workflow {
			path: layout.workflow.clone(),
			cause,
		})?;
		let broken = |rule: String| Error::Rule {
			path: layout.workflow.clone(),
			rule,
		};
		let defaults = &workflow.defaults;
		let listed = workflow
			.stages
			.iter()
			.map(|stage| stage.id.as_str())
			.eq(STAGES);
		let skilled = workflow
			.stages
			.iter()
			.map(|stage| stage.skill.as_deref())
			.eq(SKILLS);
		let priced = prices.models.iter().any(|model| model.id == defaults.model);

		if !listed {
			return Err(broken(format!(
				"its stages must be {}, in this order",
				STAGES.join(", ")
			)));
		}

		if !skilled {
			return Err(broken(String::from(
				"each stage must keep the skill nsx plays it with",
			)));
		}

		if !EFFORTS.contains(&defaults.effort.as_str()) {
			return Err(broken(format!(
				"its default effort must be one of {}",
				EFFORTS.join(", ")
			)));
		}

		if !priced {
			return Err(broken(format!(
				"its default model {} is not in the price list",
				defaults.model
			)));
		}

		Ok(workflow)
	}
}
