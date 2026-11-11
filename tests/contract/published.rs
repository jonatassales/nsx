//! One published schema, as a JSON document

use std::path::Path;

use serde_json::Value;

pub fn published(name: &str) -> Value {
	let path = Path::new(env!("CARGO_MANIFEST_DIR"))
		.join("schema")
		.join(name);
	let text = std::fs::read_to_string(path).unwrap();

	serde_json::from_str(&text).unwrap()
}
