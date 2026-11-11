//! The run at Haiku 5.5 and low effort

use crate::simulation::{diff::diff, golden::golden, pending, ran::ran};

#[tokio::test]
async fn the_run_at_haiku_and_low_effort_plays_as_the_simulation_does() {
	let differences = diff(
		&golden("run-haiku-low"),
		&ran("claude-haiku-5-5", "low", &["review"]).await,
		&pending::HAIKU,
	);

	assert!(differences.is_empty(), "{}", differences.join("\n"));
}
