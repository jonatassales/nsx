//! Waiting at a checkpoint
//!
//! One function: it takes a stage whose `waiting` line has printed and the
//! reader of the person's answers, reads one line, and gives back the
//! checkpoint confirmed, or the stop.
//!
//! ## Reads and writes
//!
//! - reads one line from the run's standard input, through the reader
//!
//! ## Rules
//!
//! - No prompt line is printed.
//! - Yes confirms. Anything else declines, and so does the end of the
//!   input: the run stops right after the `waiting` line, prints nothing
//!   more and leaves the stage `waiting` in the record (the stop rule).
//! - `seat` reads the person's "done" for the dev ticket from the same
//!   input.

use crate::flow::{checkpoint::Checkpoint, error::Error, reader::Reader};

impl Checkpoint {
	pub fn wait(stage: &str, reader: &dyn Reader) -> Result<Self, Error> {
		let declined = || Error::Declined {
			stage: String::from(stage),
		};
		let entered = reader.line().map_err(|cause| Error::Unanswered {
			stage: String::from(stage),
			cause,
		})?;
		let answer = entered.ok_or_else(declined)?;

		if !Self::confirm(&answer) {
			return Err(declined());
		}

		Ok(Self::Confirmed {
			stage: String::from(stage),
		})
	}
}
