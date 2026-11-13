//! The delivery act's lines
//!
//! One function: it takes what the run applies, its record, its money entry,
//! what the acts share and the run's execution ledger, and plays the act's
//! lines, each step calling the folder that owns the line.
//!
//! The steps, in the simulation's order:
//!
//! - `host`: bring the runtime up (`runtimeReady`)
//! - `settle`: the implementation's money (`completed`)
//! - `ship`: push the branch (`pushed`), open the pull request
//!   (`pullRequest`)
//! - `follow`: follow the pull request's checks (`checks`, `checksPassed`)
//!   and state Safe Seat (`safeSeatMet`)
//! - local: take the runtime down (`runtimeStopped`)
//! - delivery: mark the pull request ready, while money releases what was
//!   not spent (`ready`)
//! - flow: the closing comment (`next`)
//!
//! ## Rules
//!
//! - The runtime is down before the pull request is marked ready.
//! - The released amount goes onto the `ready` line and into the money
//!   ledger at the same step.
//! - `next` follows `ready` with the pull request's number, and its wording
//!   follows the Review tick.

use crate::command::{
	applied::Applied, run::Run, session::Session, work::Work,
};
use crate::evidence::record::Record;
use crate::flow::ending::Ending;
use crate::local::execution::Execution;
use crate::money::ledger::Ledger;
use crate::session::stop::Stop;

impl Run {
	pub async fn carry(
		&self,
		session: &mut Session<'_>,
		applied: &Applied,
		record: &mut Record,
		ledger: &mut Ledger,
		work: &Work,
		execution: &mut Execution,
	) -> Result<(), Stop> {
		let root = &applied.root;
		let layout = &applied.layout;
		let reviewed = applied.authorization.intent.human.review;

		let ready = self.host(session, applied, record, execution).await?;

		let settlement = self.settle(session, applied, ledger, &work.usage)?;
		let released = settlement.released;

		let (branch, pull) =
			self.ship(session, applied, record, work, released).await?;

		let (suite, verdict) = self
			.follow(session, applied, record, &branch, &pull)
			.await?;

		execution.cleanup(root, layout, session.cluster).await?;

		session.sink.print(&ready.stopped(execution).line())?;
		record
			.write(root, session.console, |run| execution.card(run))
			.await?;

		let pull = pull
			.ready(session.forge, &suite, &verdict, reviewed)
			.await?;

		session.sink.print(&pull.line(released))?;
		ledger.released_cents = Some(released.0);
		ledger.write(root, layout)?;
		record
			.write(root, session.console, |run| pull.card(run))
			.await?;

		let ending = if reviewed {
			Ending::Review
		} else {
			Ending::Merge
		};

		session.sink.print(&ending.line(pull.number))?;

		Ok(())
	}
}
