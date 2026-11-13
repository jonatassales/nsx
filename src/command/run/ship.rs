//! Pushing the work and opening its pull request
//!
//! One function: it takes what the run applies, the run's record, what the
//! acts share and what the run released, pushes the run's branch and opens
//! the draft pull request. It gives back the pushed branch and the pull
//! request.
//!
//! The steps:
//!
//! - delivery: push the branch (`pushed`), then the record's branch and
//!   commit
//! - delivery: open the draft pull request (`pullRequest`), then the
//!   record's pull request
//!
//! ## Rules
//!
//! - It runs after the `completed` line.
//! - nsx commits nothing: the push carries the committed head.

use crate::command::{
	applied::Applied, run::Run, session::Session, work::Work,
};
use crate::delivery::{branch::Branch, error::Error, pull::Pull};
use crate::evidence::record::Record;
use crate::money::cents::Cents;
use crate::session::stop::Stop;

impl Run {
	pub async fn ship(
		&self,
		session: &mut Session<'_>,
		applied: &Applied,
		record: &mut Record,
		work: &Work,
		released: Cents,
	) -> Result<(Branch, Pull), Stop> {
		let root = &applied.root;
		let branch = work.branch.clone().push(session.forge).await?;
		let pushed = branch.line().ok_or(Error::Unhashed)?;

		session.sink.print(&pushed)?;
		record
			.write(root, session.console, |run| branch.card(run))
			.await?;

		let pull = Pull::open(session.forge, &branch).await?;

		session.sink.print(&pull.line(released))?;
		record
			.write(root, session.console, |run| pull.card(run))
			.await?;

		Ok((branch, pull))
	}
}
