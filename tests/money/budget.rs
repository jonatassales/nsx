//! The budget per run, as the plan and the run each state it

use std::path::PathBuf;

use nsx::money::{budget::Budget, cents::Cents};
use nsx::session::kind::Kind;

#[test]
fn the_plan_states_the_budget_and_who_set_it() {
	let budget = Budget {
		amount: Cents(2500),
		source: PathBuf::from(".agents/0005-DD-book-a-demo-runtime.dd.md"),
	};

	let line = budget.line();

	assert_eq!(line.id, "budget");
	assert_eq!(line.kind, Kind::Output);
	assert_eq!(
		line.render(),
		"budget     $25.00 per run for model calls · set by the team in the design doc"
	);
}

#[test]
fn the_run_states_the_budget_and_cites_fin_03() {
	let budget = Budget {
		amount: Cents(2500),
		source: PathBuf::from(".agents/0005-DD-book-a-demo-runtime.dd.md"),
	};

	let line = budget.cite();

	assert_eq!(line.id, "budget");
	assert_eq!(line.kind, Kind::Output);
	assert_eq!(
		line.render(),
		"console › budget     $25.00 per run · set by the team · FIN-03"
	);
}
