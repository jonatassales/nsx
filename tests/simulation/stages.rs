//! The run with every stage checked

use crate::simulation::{diff::diff, golden::golden, pending, ran::ran};

#[tokio::test]
async fn the_run_with_every_stage_plays_as_the_simulation_does() {
	let differences = diff(
		&golden("run-all-stages"),
		&ran(
			"claude-sonnet-5-5",
			"medium",
			&["spec", "tickets", "implement", "review"],
		)
		.await,
		&pending::STAGES,
	);

	assert!(differences.is_empty(), "{}", differences.join("\n"));
}
