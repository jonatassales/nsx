//! A sample reads as its shape and writes back unchanged

use nsx::contract::decode::decode;
use pretty_assertions::assert_eq;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

use crate::contract::sample::sample;

pub fn trip<Shape: DeserializeOwned + Serialize>(name: &str) {
	let text = sample(name);
	let shape = decode::<Shape>(&text).unwrap();
	let written = serde_json::to_value(&shape).unwrap();
	let original = serde_json::from_str::<Value>(&text).unwrap();

	assert_eq!(written, original, "{name}");
}
