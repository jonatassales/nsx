//! The JSON Schemas
//!
//! The published form of these shapes, for whoever reads them without nsx: a
//! skill's author validates a stage report before a run, and the console
//! validates each report it receives. The schemas are generated from the types
//! of this folder, so a shape has one definition. The published copies live in
//! `schema/`, at the crate's root.
//!
//! One function: it takes a shape of this folder and gives back its JSON
//! Schema.
//!
//! ## Rules
//!
//! - Published: the fence, the judge's three answers, the five stage reports,
//!   `plan.json`, `intent.json` and `run.json`, and the report to the console
//!   with its answer. Spec 0073 publishes the hand-off's and the report's.
//! - A schema refuses every field its shape does not define. A section whose
//!   fields are not defined yet allows no field.
//! - A schema keeps the `$id` it is published under. The report refers to the
//!   three hand-off schemas by `$id`, and the console resolves them that way.
//! - A schema and the checks of this folder accept and refuse the same
//!   samples.
//! - What a schema cannot express stays in the checks: unique paths, distinct
//!   ports and images, shares within the bound, a reason that fits its verdict.
//! - A generated schema equals its published file as a JSON document:
//!   whitespace and key order do not count (owner, 2026-11-10).
//! - schemars spells some things its own way, and the generated schema drops
//!   them: integer formats, the type beside a closed set, and an object
//!   without a property list. A shape published on its own is referred to by
//!   its `$id`, never inlined.
//!
//! Spec 0077.

use schemars::{
	JsonSchema, Schema,
	generate::SchemaSettings,
	transform::{RecursiveTransform, Transform},
};
use serde_json::{Map, Value};

pub fn schema<Shape: JsonSchema>() -> Value {
	let generator = SchemaSettings::draft2020_12().into_generator();
	let mut root = generator.into_root_schema_for::<Shape>();
	let definitions = root.remove("$defs").unwrap_or(Value::Null);

	// A shape published on its own is referred to by its `$id`. Integer
	// formats, the type beside a closed set and a missing property list are
	// how schemars spells a shape, not part of the shape.
	let mut tidy = RecursiveTransform(|schema: &mut Schema| {
		let published = schema
			.get("$ref")
			.and_then(Value::as_str)
			.and_then(|reference| reference.strip_prefix("#/$defs/"))
			.and_then(|name| definitions.get(name))
			.and_then(|definition| definition.get("$id"))
			.cloned();

		if let Some(id) = published {
			schema.insert(String::from("$ref"), id);
		}

		schema.remove("format");

		let closed = schema.get("const").is_some() || schema.get("enum").is_some();

		if closed {
			schema.remove("type");
		}

		let object = schema.get("type").and_then(Value::as_str) == Some("object");

		if object && schema.get("properties").is_none() {
			schema.insert(String::from("properties"), Value::Object(Map::new()));
		}
	});

	tidy.transform(&mut root);

	root.to_value()
}
