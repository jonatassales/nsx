//! The stop of an interrupted command
//!
//! One function: it gives back the stop a command ends on when the person
//! interrupts it.
//!
//! ## Rules
//!
//! - An interrupt is a stop like any other: one sentence, exit 1.

use crate::session::stop::Stop;

impl Stop {
	pub fn interrupted() -> Self {
		Self {
			sentence: String::from("nsx was interrupted."),
		}
	}
}
