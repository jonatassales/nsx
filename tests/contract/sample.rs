//! One of Core's samples, as copied beside these tests

use std::path::Path;

pub fn sample(name: &str) -> String {
	let path = Path::new(env!("CARGO_MANIFEST_DIR"))
		.join("tests/contract/samples")
		.join(name);

	std::fs::read_to_string(path).unwrap()
}
