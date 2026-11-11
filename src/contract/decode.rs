//! Strict decoding
//!
//! How every record of this folder is read from JSON text: the hand-off
//! records, the judge's answers, the stage reports and the report to the
//! console. A record that carries more than its shape defines is refused, not
//! trimmed.
//!
//! One function: it takes a JSON text and the shape expected, and gives back
//! the record, or refuses the text.
//!
//! ## Rules
//!
//! - A field the shape does not define is refused.
//! - A text that holds more than one record is refused.
//! - A `null` stays unknown. It is never read as zero, empty or false.
//!
//! Spec 0077.

use serde::de::DeserializeOwned;

use crate::contract::error::Error;

pub fn decode<Record: DeserializeOwned>(text: &str) -> Result<Record, Error> {
	let mut reader = serde_json::Deserializer::from_str(text);

	let record =
		Record::deserialize(&mut reader).map_err(|cause| Error::Record {
			cause,
		})?;

	if reader.end().is_err() {
		return Err(Error::Records);
	}

	Ok(record)
}
