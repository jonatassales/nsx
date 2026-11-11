//! The simulation's proposal: the workflow's default choice at a revision

use nsx::contract::human::Human;
use nsx::flow::choice::Choice;
use nsx::handoff::proposal::Proposal;

pub fn proposed(revision: i64) -> Proposal {
	Proposal {
		ticket: String::from("0031-TICKET-book-a-demo-runtime"),
		runtime: String::from("book-a-demo-runtime"),
		revision,
		choice: Choice {
			model: String::from("claude-sonnet-5-5"),
			effort: String::from("medium"),
			human: Human {
				planning: true,
				spec: false,
				tickets: false,
				implement: false,
				review: true,
			},
		},
	}
}
