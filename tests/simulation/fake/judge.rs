//! The judge of the golden test: it answers as the simulation's judge did
//!
//! The answers are the samples kept beside the contract's tests.

use std::path::Path;

use crate::simulation::fake::recorded::Recorded;

pub fn judge() -> Recorded {
	let samples = Path::new(env!("CARGO_MANIFEST_DIR"))
		.join("tests/contract/samples/judgment");

	let recorded =
		|name: &str| std::fs::read_to_string(samples.join(name)).unwrap();

	Recorded {
		criteria: recorded("criteria.answer.sample.json"),
		episodes: recorded("episodes.answer.sample.json"),
		suggestion: recorded("suggestion.answer.sample.json"),
	}
}
