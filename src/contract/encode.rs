//! Writing a record as JSON text
//!
//! One function: it takes a record and gives back its JSON text, indented
//! with tabs, as every file nsx writes is.
//!
//! ## Rules
//!
//! - The fields are written in the order the type declares them.
//! - The text carries no final newline: the file's writer adds it.
//!
//! Spec 0077.

use serde::Serialize;
use serde::ser::Error;
use serde_json::{Serializer, ser::PrettyFormatter};

pub fn encode<Record: Serialize>(
	record: &Record,
) -> Result<String, serde_json::Error> {
	let mut text = Vec::new();
	let mut serializer =
		Serializer::with_formatter(&mut text, PrettyFormatter::with_indent(b"\t"));

	record.serialize(&mut serializer)?;

	String::from_utf8(text).map_err(serde_json::Error::custom)
}
