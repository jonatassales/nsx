//! Recording a part of the cards
//!
//! One function: it takes the project's root, the console and what fills a
//! part of the cards. It writes `plan.json` whole, then sends the console
//! the cards as they now stand.
//!
//! ## Rules
//!
//! - The file is written before anything is sent: the console is told only
//!   what the project holds.
//! - A console that does not answer stops nothing (`console/report/send.rs`).
//!
//! Spec 0077.

use crate::command::desk::Desk;
use crate::console::client::Client;
use crate::contract::cards::Cards;
use crate::files::root::Root;
use crate::session::stop::Stop;

impl Desk {
	pub async fn record(
		&mut self,
		root: &Root,
		console: &dyn Client,
		fill: impl FnOnce(&mut Cards),
	) -> Result<(), Stop> {
		self.writer.write(root, fill)?;

		self
			.report
			.send(console, Some(&self.writer.cards), None, None)
			.await?;

		Ok(())
	}
}
