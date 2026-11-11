//! Holding before a stage
//!
//! One function: it takes the reports' state, the console, the records
//! written so far and the pace of a hold. It asks the console whether the
//! team paused the run and, if so, waits for the team's Play. It gives back
//! when the stage's work may start.
//!
//! ## Reads and writes
//!
//! - calls the console once at the hold point, reporting `workflow`, then
//!   at once and at each beat while held, reporting `held`
//!
//! ## Rules
//!
//! - At the hold point it sends the record and reads the answer before the
//!   stage's work starts. Sends are awaited in turn, so none is in flight.
//! - It waits at most the client's 4-second timeout for that answer. An
//!   answer that fails or times out counts as no pause, and the run goes on.
//! - While held it prints nothing. An answer that requests play resumes the
//!   stage's work, and the status returns to `workflow`.
//! - A held run never resumes on its own, even when the console stops
//!   answering. There is no timeout.
//! - With no connection, nothing is sent, so nothing holds.
//!
//! Spec 0081.

use std::time::Duration;

use crate::console::{
	client::Client, error::Error, hold::Hold, report::Report,
};
use crate::contract::{
	asked::Asked, cards::Cards, intent::Intent, run::Run, status::Status,
};

impl Hold {
	pub async fn wait(
		&self,
		report: &mut Report,
		client: &dyn Client,
		records: (Option<&Cards>, Option<&Intent>, Option<&Run>),
		beat: Duration,
	) -> Result<(), Error> {
		// What the console asked, if it answered: any failure is no answer.
		async fn asked(
			report: &mut Report,
			client: &dyn Client,
			records: (Option<&Cards>, Option<&Intent>, Option<&Run>),
		) -> Result<Option<Asked>, Error> {
			let (cards, intent, run) = records;

			match report.ask(client, cards, intent, run).await {
				Ok(requested) => Ok(requested.map(|requested| requested.state)),
				Err(Error::File(unwritten)) => Err(Error::File(unwritten)),
				Err(_unanswered) => Ok(None),
			}
		}

		report.status = Status::Workflow;

		let first = asked(report, client, records).await?;

		if first != Some(Asked::Pause) {
			return Ok(());
		}

		report.status = Status::Held;

		// `held` is told at once, then at each beat, until the team's Play.
		loop {
			let answer = asked(report, client, records).await?;

			if answer == Some(Asked::Play) {
				report.status = Status::Workflow;

				return Ok(());
			}

			tokio::time::sleep(beat).await;
		}
	}
}
