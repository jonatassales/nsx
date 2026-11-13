//! Starting the branch
//!
//! One function: it takes the forge and the plan's ticket, starts the branch
//! from the checked-out commit, and gives back the branch.
//!
//! ## Reads and writes
//!
//! - calls the forge, which runs `git` to start the branch, or to check it
//!   out when it exists
//!
//! ## Rules
//!
//! - It runs after the gate opens and before the first stage, and prints no
//!   line.
//! - When the branch already exists from an earlier run of the same ticket,
//!   the run checks it out and works on it instead.
//! - A branch that can be neither started nor checked out stops the run
//!   before the first stage (the stop rule). So does a ticket with no
//!   number: its branch has no name.
//! - No branch is started in an over-budget run.

use crate::contract::ticket::Ticket;
use crate::delivery::{branch::Branch, error::Error, forge::Forge};

impl Branch {
	pub async fn start(
		forge: &dyn Forge,
		ticket: &Ticket,
	) -> Result<Self, Error> {
		let number = ticket.number.as_ref().ok_or_else(|| Error::Numberless {
			ticket: ticket.id.clone(),
		})?;
		let name = format!("feat/{number}-{}", ticket.slug);

		forge.start(&name).await?;

		Ok(Self {
			name,
			commit: None,
		})
	}
}
