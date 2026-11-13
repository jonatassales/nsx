//! The delivery act of the run
//!
//! After the code review, the runtime comes up, the work is pushed and a
//! pull request opened, the client's checks are followed, and the runtime is
//! taken down before the pull request is marked ready.
//!
//! One function: it takes what the run applies, its record, its money entry
//! and what the acts share, and plays the act. It gives back nothing once
//! the last line has printed, or the stop.
//!
//! The steps:
//!
//! - local: open the run's execution ledger, with the run's space
//! - `carry`: the act's lines, from `runtimeReady` to `next`
//!
//! ## Rules
//!
//! - The runtime comes up only after the code review.
//! - The ledger is opened before the cluster is asked for anything, so a
//!   stop from here on finds what to take down (`halt`).
//! - Nothing in nsx merges: the merge stays the owner's.

use crate::command::{
	applied::Applied, run::Run, session::Session, work::Work,
};
use crate::evidence::record::Record;
use crate::local::execution::Execution;
use crate::money::ledger::Ledger;
use crate::session::stop::Stop;

impl Run {
	pub async fn deliver(
		&self,
		session: &mut Session<'_>,
		applied: &Applied,
		record: &mut Record,
		ledger: &mut Ledger,
		work: &Work,
	) -> Result<(), Stop> {
		let runtime = &applied.authorization.intent.runtime_id;
		let mut execution = Execution::open(
			&applied.root,
			&applied.layout,
			&applied.id.text(),
			runtime,
		)?;

		self
			.carry(session, applied, record, ledger, work, &mut execution)
			.await
	}
}
