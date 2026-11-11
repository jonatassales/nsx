//! The run whose reservation exceeds the budget: it pauses at the gate

use crate::simulation::{diff::diff, golden::golden, pending, ran::ran};

#[tokio::test]
async fn the_run_over_its_budget_plays_as_the_simulation_does() {
	let differences = diff(
		&golden("run-over-budget"),
		&ran("claude-opus-5-5", "high", &["review"]).await,
		&pending::BUDGET,
	);

	assert!(differences.is_empty(), "{}", differences.join("\n"));
}
