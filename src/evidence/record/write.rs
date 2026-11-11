//! Writing the run's record
//!
//! One function: it takes the project's root, the console and what fills a
//! part of the record. It writes `run.json` whole, then sends the console
//! the plan's cards, the applied intent and the record as they now stand.
//!
//! ## Reads and writes
//!
//! - writes `.langdrift/microruntimes/runtimes/<runtime id>/run.json`
//! - calls the console's report tool, through the report's `send`
//!
//! ## Rules
//!
//! - The file is written before anything is sent: the console is told only
//!   what the project holds.
//! - A console that does not answer stops nothing (`console/report/send.rs`).
//!
//! Spec 0080.

use crate::console::client::Client;
use crate::contract::{encode::encode, run::Run};
use crate::evidence::{error::Error, record::Record};
use crate::files::{inside::inside, root::Root, write::write};

impl Record {
	pub async fn write(
		&mut self,
		root: &Root,
		console: &dyn Client,
		fill: impl FnOnce(&mut Run),
	) -> Result<(), Error> {
		fill(&mut self.run);

		let file = inside(root, &self.file)?;
		let mut text = encode(&self.run).map_err(|cause| Error::Unwritten {
			path: file.clone(),
			cause,
		})?;

		text.push('\n');
		write(&file, text.as_bytes())?;

		self
			.report
			.send(
				console,
				Some(&self.cards),
				self.run.intent.as_ref(),
				Some(&self.run),
			)
			.await?;

		Ok(())
	}
}
