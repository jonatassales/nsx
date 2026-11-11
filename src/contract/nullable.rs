//! A required field that may be `null`
//!
//! What the schema of a field says when the field is always present but its
//! value may not be known yet: the ticket's unstated number, status and title,
//! and the run's id and outcome. schemars drops `null` from an optional field
//! it is told is required, so this adds it back.
//!
//! One function: it takes the schema of such a field and lets it be `null`.
//!
//! Spec 0077.

use schemars::Schema;
use serde_json::Value;

pub fn nullable(schema: &mut Schema) {
	let fields = schema.ensure_object();

	if let Some(Value::Array(values)) = fields.get_mut("enum") {
		values.push(Value::Null);
	}

	if let Some(kind) = fields.remove("type") {
		let kinds = Value::Array(vec![kind, Value::from("null")]);

		fields.insert(String::from("type"), kinds);
	}
}
