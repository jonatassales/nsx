//! Telling the console how a command ended
//!
//! One function: it takes the status the command ends on, the console and
//! the records written so far, and tells the console once, whether or not
//! an earlier call failed.
//!
//! ## Reads and writes
//!
//! - calls the console's report tool, through `ask`
//!
//! ## Rules
//!
//! - The console's answer, or its silence, changes nothing: a command that
//!   ends has nothing left to apply.
//! - Only keeping the console's id can fail, as a file.

use crate::console::{client::Client, error::Error, report::Report};
use crate::contract::{cards::Cards, intent::Intent, run::Run, status::Status};

impl Report {
	pub async fn end(
		&mut self,
		status: Status,
		client: &dyn Client,
		records: (Option<&Cards>, Option<&Intent>, Option<&Run>),
	) -> Result<(), crate::files::error::Error> {
		let (cards, intent, run) = records;

		self.status = status;

		match self.ask(client, cards, intent, run).await {
			Err(Error::File(unwritten)) => Err(unwritten),
			_answered => Ok(()),
		}
	}
}
