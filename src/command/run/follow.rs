//! Following the checks, then Safe Seat
//!
//! One function: it takes what the run applies, the run's record, the
//! pushed branch and the run's pull request, follows the client's checks
//! until they pass and states Safe Seat. It gives back the checks and the
//! verdict.
//!
//! The steps:
//!
//! - checks: the first listing (`checks`), then the record's checks
//! - checks: the poll until every check passed (`checksPassed`)
//! - seat: the verdict, from the dev ticket as the record closed it
//!   (`safeSeatMet`), then the record's Safe Seat section
//!
//! ## Rules
//!
//! - The checks followed are those of the commit the run pushed, and no
//!   other's.
//! - A check that does not pass stops the run before `checksPassed`.
//! - Safe Seat is stated only after every check passed. A verdict that is
//!   not met prints no line and stops the run: the pull request stays a
//!   draft.

use crate::checks::suite::Suite;
use crate::command::{applied::Applied, run::Run, session::Session};
use crate::delivery::{
	branch::Branch, error::Error as Undelivered, pull::Pull,
};
use crate::evidence::record::Record;
use crate::seat::{
	error::Error,
	part::{Part, STAGE},
	verdict::Verdict,
};
use crate::session::stop::Stop;

impl Run {
	pub async fn follow(
		&self,
		session: &mut Session<'_>,
		applied: &Applied,
		record: &mut Record,
		branch: &Branch,
		pull: &Pull,
	) -> Result<(Suite, Verdict), Stop> {
		let root = &applied.root;
		let commit = branch.commit.as_deref().ok_or(Undelivered::Unhashed)?;
		let pace = session.interval;
		let listed = Suite::list(session.forge, pull.number, commit, pace).await?;

		session.sink.print(&listed.line(false))?;
		record
			.write(root, session.console, |run| listed.card(false, run))
			.await?;

		let suite = listed
			.poll(session.forge, pull.number, commit, pace)
			.await?;

		session.sink.print(&suite.line(true))?;
		record
			.write(root, session.console, |run| suite.card(true, run))
			.await?;

		let closed = record
			.run
			.stages
			.iter()
			.flatten()
			.find(|step| step.id == STAGE)
			.ok_or(Error::Unmet)?;
		let verdict = Verdict::reach(&Part::of(closed), session.clock);
		let met = verdict.line().ok_or(Error::Unmet)?;

		session.sink.print(&met)?;
		record
			.write(root, session.console, |run| verdict.card(run))
			.await?;

		Ok((suite, verdict))
	}
}
