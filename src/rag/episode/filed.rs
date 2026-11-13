//! The id in a record's file name
//!
//! One function: it takes a file's name and gives back the id of the
//! episode it records, when the name is a record's: `<id>-<slug>.json`.
//!
//! ## Rules
//!
//! - The id is the name up to its second hyphen, and it is an episode's id
//!   (`number`).
//! - Any other name in the episodes folder is no record.

use crate::rag::episode::Episode;

impl Episode {
	pub fn filed(name: &str) -> Option<&str> {
		let stem = name.strip_suffix(".json")?;
		let (at, _hyphen) = stem.match_indices('-').nth(1)?;
		let id = &stem[..at];

		Self::number(id)?;

		Some(id)
	}
}
