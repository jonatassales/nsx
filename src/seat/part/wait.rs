//! Waiting for the person's part
//!
//! One function: it takes the dev-ticket stage once its `waiting` line has
//! printed and the reader of the person's answers, and reads one line. It
//! gives back nothing, or the stop when the input ended.
//!
//! ## Reads and writes
//!
//! - reads one line from the run's standard input, through the reader
//!
//! ## Rules
//!
//! - It is the same input as a checkpoint confirmation, inside the same
//!   `nsx agent run`.
//! - While it waits, the run makes no call to the coding agent.
//! - When the input ends, the run stops after the `waiting` line: it prints
//!   nothing more, makes no call and records the stage as `active` (the stop
//!   rule).
//!
//! ## Open
//!
//! - How nsx knows the person finished. Until the owner decides, the person
//!   enters a line in the run's terminal. A commit on the branch or a change
//!   in the reserved part's paths are other possible signals.

use crate::flow::reader::Reader;
use crate::seat::{error::Error, part::Part};

impl Part {
	pub fn wait(&self, reader: &dyn Reader) -> Result<(), Error> {
		let entered = reader.line().map_err(|cause| Error::Unread {
			cause,
		})?;

		// Any line says the person is done: only the end of the input does
		// not.
		if entered.is_none() {
			return Err(Error::Unanswered);
		}

		Ok(())
	}
}
