//! The words a stage call gives the coding agent
//!
//! One function: it gives back the prompt of the call: the stage's skill,
//! the ticket, the effort and where the skill writes its report.
//!
//! ## Rules
//!
//! - It names the skill text, the ticket, the effort and the instruction to
//!   write the stage's report at the report path, and nothing else: what a
//!   skill does is the skill's own text, in `.langdrift/skills/`.
//!
//! Spec 0080.

use crate::driver::call::Call;

impl Call {
	pub fn prompt(&self) -> String {
		format!(
			"Follow the LangDrift skill {} for the ticket {}, at {} effort. When the stage's work is done, write the stage's report as one JSON object at {}.",
			self.skill,
			self.ticket,
			self.effort,
			self.report.display()
		)
	}
}
