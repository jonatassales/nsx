//! The console's record of the runtime a run planned
//!
//! Read from the console's API with the id nsx kept in the project's
//! private state.

use std::path::Path;

use serde_json::Value;

pub async fn recorded(project: &Path, console: &str, slug: &str) -> Value {
	let kept = project
		.join(".langdrift/agent/state/console")
		.join(format!("{slug}.id"));
	let id = std::fs::read_to_string(kept).unwrap();
	let address = format!("{console}/api/console/runtimes/{}", id.trim());

	let answered = reqwest::get(address).await.unwrap();

	let body = answered.json::<Value>().await.unwrap();

	body["runtime"].clone()
}
