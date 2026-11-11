//! The stage's line
//!
//! One function: it takes a stage's checked report and the short id of the
//! spec the ticket links, and gives back the stage's one line, built from
//! the report's figures in the simulation's exact wording.
//!
//! ## Simulation
//!
//! - `grilling`: `console › grilling   3 open questions settled`
//! - `toSpec`: `console › to-spec    0011-SPEC revised · 3 acceptance criteria`
//! - `toTickets`: `console › to-tickets 3 tickets · blocking edges recorded`
//! - `implement`: `console › implement  tdd · 5 components · 14 files · boundary only · stops at the reserved part`
//! - `codeReview`: `console › code-review 2 findings · fixed`
//!
//! ## Rules
//!
//! - A line prints only once its call has ended and its report is checked.
//! - The counts come from the report. The rest of each line is fixed text.
//! - `to-spec` names the spec by its short id, `0011-SPEC`, under spec 0053's
//!   rule for linked documents.
//! - A count of 1 takes the singular noun: "1 ticket", "1 finding",
//!   "1 acceptance criterion", "1 open question", "1 component", "1 file".
//! - `code-review` never has a `waiting` line before it, whether or not
//!   Review is ticked.
//! - No card field. The card's stage states (spec 0057) turn on these lines.
//!
//! Spec 0080.

use crate::contract::figures::Figures;
use crate::session::{kind::Kind, line::Line, shape::Shape};
use crate::skills::report::Report;

impl Report {
	pub fn line(&self, spec: &str) -> Line {
		// A count with its noun: one takes the singular.
		let counted = |count: u64, one: &str, many: &str| match count {
			1 => format!("1 {one}"),
			_ => format!("{count} {many}"),
		};
		let (id, label, value) = match &self.figures {
			Figures::Planning(report) => (
				"grilling",
				"grilling",
				format!(
					"{} settled",
					counted(
						report.open_questions_settled,
						"open question",
						"open questions"
					)
				),
			),
			Figures::Spec(report) => (
				"toSpec",
				"to-spec",
				format!(
					"{spec} revised · {}",
					counted(
						report.acceptance_criteria,
						"acceptance criterion",
						"acceptance criteria"
					)
				),
			),
			Figures::Tickets(report) => (
				"toTickets",
				"to-tickets",
				format!(
					"{} · blocking edges recorded",
					counted(report.tickets, "ticket", "tickets")
				),
			),
			Figures::Implement(report) => (
				"implement",
				"implement",
				format!(
					"tdd · {} · {} · boundary only · stops at the reserved part",
					counted(report.components, "component", "components"),
					counted(report.files, "file", "files")
				),
			),
			Figures::Review(report) => (
				"codeReview",
				"code-review",
				format!(
					"{} · fixed",
					counted(report.findings, "finding", "findings")
				),
			),
		};

		Line {
			id: String::from(id),
			kind: Kind::Output,
			shape: Shape::Labeled {
				label: String::from(label),
				value,
			},
		}
	}
}
