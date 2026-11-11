//! The run with Planning and Review, to review

use crate::simulation::{diff::diff, golden::golden, pending, ran::ran};

#[tokio::test]
async fn the_run_plays_as_the_simulation_does() {
	let differences = diff(
		&golden("run"),
		&ran("claude-sonnet-5-5", "medium", &["review"]).await,
		&pending::RUN,
	);

	assert!(differences.is_empty(), "{}", differences.join("\n"));
}
