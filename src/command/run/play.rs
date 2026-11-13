//! The run's acts, after its opening
//!
//! One function: it takes what the run applies, the run's record and its
//! money entry, and plays the acts in order. It gives back the outcome the
//! run ended on, or the stop.
//!
//! The acts, in the simulation's order:
//!
//! - `reserve`: the money act, from `estimate` to the gate
//! - `prepare`: what the stages need, and the run's branch, with no line
//! - `stages`: the stages, from `waiting-planning` to `codeReview`
//! - `deliver`: the delivery act, from `runtimeReady` to `next`
//!
//! ## Rules
//!
//! - A gate that paused ends the run there, as `paused`: no stage plays and
//!   no branch is started.
//! - A run that plays its last line ends `completed`.

use crate::command::{applied::Applied, run::Run, session::Session};
use crate::contract::outcome::Outcome;
use crate::evidence::record::Record;
use crate::money::{gate::Gate, ledger::Ledger};
use crate::session::stop::Stop;

impl Run {
	pub async fn play(
		&self,
		session: &mut Session<'_>,
		applied: &Applied,
		record: &mut Record,
		ledger: &mut Ledger,
	) -> Result<Outcome, Stop> {
		let gate = self.reserve(session, applied, ledger)?;
		let paused = matches!(gate, Gate::Paused { .. });

		if paused {
			return Ok(Outcome::Paused);
		}

		let mut work = self.prepare(session, applied).await?;

		self.stages(session, applied, record, &mut work).await?;

		self
			.deliver(session, applied, record, ledger, &work)
			.await?;

		Ok(Outcome::Completed)
	}
}
