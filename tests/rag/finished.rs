//! The episode a run of a notes runtime would leave

use nsx::rag::{episode::Episode, part::Part, resource::Resource};

pub const INTENT: &str = "Run the notes service as a microruntime\nThe notes service runs as one bounded microruntime: an API and its cache.";

pub fn finished() -> Episode {
	Episode {
		id: String::new(),
		slug: String::from("notes-runtime"),
		ticket: String::from("0032-TICKET-notes-runtime"),
		intent: String::from(INTENT),
		components: vec![
			Part {
				path: String::from("services/notes-api"),
				stack: String::from("Rust"),
				role: String::from("api"),
			},
			Part {
				path: String::from("platform/redis"),
				stack: String::from("Redis"),
				role: String::from("cache"),
			},
		],
		resource: Resource {
			kind: String::from("Deployment"),
			count: 2,
			replicas: 1,
		},
		measured: None,
	}
}
