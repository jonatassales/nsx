//! What a run needs before its first stage
//!
//! One function: it takes what the run applies, checks what the stages will
//! need while nothing is owed, starts the run's branch and gives back what
//! the acts share.
//!
//! The steps:
//!
//! - driver: read the driver configuration
//! - skills: find the spec the ticket links
//! - evidence: read the usage record, before any paid call
//! - files: keep the run's private state out of Git
//! - local: remove what earlier stopped runs of this runtime left
//! - delivery: start the run's branch, or check it out, with no line
//!
//! ## Rules
//!
//! - It runs after the gate opens and before any stage waits: a run over its
//!   budget never reaches here, and still pauses without a driver
//!   configuration.
//! - The checks come first: a run that would stop here leaves the person on
//!   the branch they were on.
//! - What an earlier run left in the cluster is removed here, with no line,
//!   and not at the bring-up: a leftover that cannot be removed stops the
//!   run before anything is paid.
//! - A branch that can be neither started nor checked out stops the run
//!   before the first stage.

use crate::command::{
	applied::Applied, run::Run, session::Session, work::Work,
};
use crate::delivery::{branch::Branch, error::Error as Undelivered};
use crate::driver::config::Config;
use crate::evidence::usage::Usage;
use crate::files::hide::hide;
use crate::local::execution::Execution;
use crate::session::stop::Stop;
use crate::skills::error::Error;

impl Run {
	pub async fn prepare(
		&self,
		session: &mut Session<'_>,
		applied: &Applied,
	) -> Result<Work, Stop> {
		let root = &applied.root;
		let intent = &applied.authorization.intent;
		let config = Config::load(root, &applied.layout)?;
		let ticket = applied
			.cards
			.ticket
			.as_ref()
			.ok_or(Undelivered::Unplanned)?;
		let spec = ticket
			.links
			.iter()
			.flatten()
			.find(|linked| linked.kind == "spec")
			.cloned()
			.ok_or(Error::Unlinked)?;
		let folder = applied.layout.runtimes.join(&intent.runtime_id);
		let file = folder.join("usage.json");

		// A usage record that does not read stops the run here, while nothing
		// is owed: never after a call it could not keep.
		Usage::kept(root, &file)?;

		hide(root, &applied.layout)?;

		Execution::sweep(
			root,
			&applied.layout,
			&intent.runtime_id,
			session.cluster,
		)
		.await?;

		let branch = Branch::start(session.forge, ticket).await?;

		Ok(Work {
			config,
			spec,
			usage: Usage {
				run: applied.id.text(),
				model: intent.model.clone(),
				effort: intent.effort.clone(),
				calls: Vec::new(),
				input_tokens: None,
				output_tokens: None,
			},
			file,
			effort: None,
			branch,
		})
	}
}
