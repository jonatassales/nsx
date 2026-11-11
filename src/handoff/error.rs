//! What the hand-off can refuse
//!
//! The errors of this folder, each one plain sentence that names the record,
//! so that the stop rule can print it as it is.
//!
//! Definition only: the `Error` type.
//!
//! Spec 0079.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("nsx could not write {} as JSON: {cause}.", path.display())]
	Unwritten {
		path: PathBuf,
		cause: serde_json::Error,
	},
	#[error(
		"nsx will not write the intent {}: {problems}.",
		path.display()
	)]
	Invalid { path: PathBuf, problems: String },
	#[error("nsx will not run {ticket}: {rule}.")]
	Unauthorized { ticket: String, rule: String },
	#[error(
		"nsx found no intent to run: plan a ticket, and have the team authorize it."
	)]
	Nothing,
	#[error(
		"nsx found several authorized intents and will not pick one: {tickets}."
	)]
	Several { tickets: String },
	#[error("nsx could not read {}: {cause}.", path.display())]
	Unread {
		path: PathBuf,
		cause: crate::contract::error::Error,
	},
	#[error("{cause}")]
	Draft {
		cause: crate::contract::error::Error,
	},
	#[error(
		"nsx will not run the intent in {}: it names another runtime than its folder.",
		path.display()
	)]
	Misplaced { path: PathBuf },
	#[error(transparent)]
	File(#[from] crate::files::error::Error),
}
