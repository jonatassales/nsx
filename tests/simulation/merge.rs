//! The run without Review, ready to merge

use crate::simulation::{diff::diff, golden::golden, pending, ran::ran};

#[tokio::test]
async fn the_run_to_merge_plays_as_the_simulation_does() {
	let differences = diff(
		&golden("run-to-merge"),
		&ran("claude-sonnet-5-5", "medium", &[]).await,
		&pending::MERGE,
	);

	assert!(differences.is_empty(), "{}", differences.join("\n"));
}
