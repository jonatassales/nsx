//! The dev ticket
//!
//! One function: it takes what the run applies, the run's record and what
//! the stages share, and plays the dev-ticket stage: the run hands the
//! reserved part to the person, waits, and closes the stage.
//!
//! The steps:
//!
//! - seat: the waiting line (`devTicketWaiting`)
//! - seat: wait for the person's line on standard input
//! - seat: close the stage, with the calls made meanwhile (`devTicketClosed`)
//!
//! ## Rules
//!
//! - Each write of the stage into `run.json` is told to the console
//!   (`evidence/record/write.rs`).
//!
//! - No call to the coding agent is made between the two lines. The count
//!   recorded is measured: the run's calls in the usage record on the disk,
//!   read before the wait and after it.
//! - A run that paid for a call meanwhile stops before the `closed` line,
//!   which says "no paid call". So does one whose usage record holds fewer
//!   of its calls than before the wait: that count measures nothing.

use crate::command::{
	applied::Applied, run::Run, session::Session, work::Work,
};
use crate::contract::step::ACTIVE;
use crate::evidence::{record::Record, usage::Usage};
use crate::seat::{
	error::Error,
	part::{Part, STAGE},
};
use crate::session::stop::Stop;

impl Run {
	pub async fn person(
		&self,
		session: &mut Session<'_>,
		applied: &Applied,
		record: &mut Record,
		work: &Work,
	) -> Result<(), Stop> {
		let root = &applied.root;
		let run = applied.id.text();
		// What this run has paid for, as the usage record on the disk says.
		let paid = || {
			let runs = Usage::kept(root, &work.file)?;
			let calls = runs
				.iter()
				.filter(|usage| usage.run == run)
				.map(|usage| usage.calls.len())
				.sum::<usize>();

			Ok::<usize, Stop>(calls)
		};
		let before = paid()?;
		let waiting = Part {
			state: String::from(ACTIVE),
			closed_by: None,
			paid_calls: None,
		};

		session.sink.print(&waiting.line())?;
		record
			.stage(root, session.console, STAGE, |step| {
				step.state = String::from(ACTIVE)
			})
			.await?;

		waiting.wait(session.answers)?;

		let after = paid()?;
		let during = after.checked_sub(before).ok_or(Error::Lost {
			before,
			after,
		})?;

		// The closed line says "no paid call": it prints only when that is so.
		if during != 0 {
			return Err(Stop::from(Error::Paid {
				calls: during,
			}));
		}

		let closed = waiting.close(during);

		session.sink.print(&closed.line())?;
		record
			.stage(root, session.console, STAGE, |step| {
				step.state = closed.state.clone();
				step.closed_by = closed.closed_by.clone();
				step.paid_calls = closed.paid_calls;
			})
			.await?;

		Ok(())
	}
}
