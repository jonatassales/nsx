//! Sending a report
//!
//! One function: it takes the console and the records written so far, and
//! sends one report as a record is written.
//!
//! ## Reads and writes
//!
//! - calls the console's report tool, through `ask`
//!
//! ## Rules
//!
//! - It prints nothing, whatever the console answers.
//! - A call that fails makes the console unavailable: later sends do
//!   nothing until an ask at a hold point is answered (`ask`). It stops
//!   nothing.
//! - The team's request in the answer is not applied here: a run applies it
//!   only at its hold points and while it waits (`console/play`,
//!   `console/hold`).
//! - Only keeping the console's id can fail the command: a state file nsx
//!   cannot write is a stop, as for any file of the project.
//!
//! Spec 0081.

use crate::console::{client::Client, error::Error, report::Report};
use crate::contract::{cards::Cards, intent::Intent, run::Run};

impl Report {
	pub async fn send(
		&mut self,
		client: &dyn Client,
		cards: Option<&Cards>,
		intent: Option<&Intent>,
		run: Option<&Run>,
	) -> Result<(), crate::files::error::Error> {
		if self.unavailable {
			return Ok(());
		}

		match self.ask(client, cards, intent, run).await {
			Ok(_requested) => Ok(()),
			Err(Error::File(unwritten)) => Err(unwritten),
			Err(_unavailable) => {
				self.unavailable = true;

				Ok(())
			}
		}
	}
}
