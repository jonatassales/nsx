//! The run's id, as text
//!
//! One function: it gives back the id as files and records carry it: the
//! runtime id and the run's number as four digits.
//!
//! Spec 0080.

use crate::evidence::id::Id;

impl Id {
	pub fn text(&self) -> String {
		format!("{}-{:04}", self.runtime, self.number)
	}
}
