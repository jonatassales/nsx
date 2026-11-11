//! The simulation's intent, as a run applies it

use nsx::contract::{human::Human, intent::Intent};

pub fn applied() -> Intent {
	Intent {
		ticket: String::from("0031-TICKET-book-a-demo-runtime"),
		runtime_id: String::from("book-a-demo-runtime"),
		revision: 1,
		model: String::from("claude-sonnet-5-5"),
		effort: String::from("medium"),
		human: Human {
			planning: true,
			spec: false,
			tickets: false,
			implement: false,
			review: true,
		},
		status: String::from("authorized"),
	}
}
