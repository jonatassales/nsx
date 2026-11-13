//! What the retrieval can refuse
//!
//! The errors of this folder, each one plain sentence that names the file, so
//! that the stop rule can print it as it is.
//!
//! Definition only: the `Error` type.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("nsx could not read the episode {}: {cause}.", path.display())]
	Episode {
		path: PathBuf,
		cause: crate::contract::error::Error,
	},
	#[error("nsx could not read the episode index {}: {cause}.", path.display())]
	Index {
		path: PathBuf,
		cause: crate::contract::error::Error,
	},
	#[error(
		"nsx could not read the episode index {}: the vector of {id} is not as wide as the embedder's.",
		path.display()
	)]
	Width { path: PathBuf, id: String },
	#[error("nsx could not search {}: {cause}.", path.display())]
	Unsearchable {
		path: PathBuf,
		cause: std::io::Error,
	},
	#[error("nsx could not write {} as JSON: {cause}.", path.display())]
	Unwritten {
		path: PathBuf,
		cause: serde_json::Error,
	},
	#[error(
		"nsx has no words yet for a cited episode that measured resources: {cited} of the cited ones did."
	)]
	Measured { cited: u64 },
	#[error(
		"nsx cannot write the run's episode: the plan's cards hold no {what}."
	)]
	Unplanned { what: &'static str },
	#[error(
		"nsx cannot write the run's episode: the plan suggests {count} replicas, which is no count an episode keeps."
	)]
	Replicas { count: i64 },
	#[error(
		"nsx could not rebuild the episode index: the record {} holds the episode {id}, which its name does not say.",
		path.display()
	)]
	Misnamed { path: PathBuf, id: String },
	#[error(
		"nsx will not write an episode for {slug:?}: a runtime's id is a lowercase label."
	)]
	Slug { slug: String },
	#[error("nsx has no episode number left for the project.")]
	Exhausted,
	#[error(transparent)]
	File(#[from] crate::files::error::Error),
}
