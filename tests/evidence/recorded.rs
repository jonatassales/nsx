//! A run's record as it opens: its id, and no section yet

use nsx::contract::{outcome::Outcome, run::Run};

pub fn recorded() -> Run {
	Run {
		run: Some(String::from("book-a-demo-runtime-0001")),
		intent: None,
		stages: None,
		effort: None,
		safe_seat: None,
		runtime: None,
		delivery: None,
		outcome: Some(Outcome::Running),
	}
}
