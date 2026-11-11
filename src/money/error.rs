//! What reading the money files can refuse
//!
//! The errors of this folder, each one plain sentence, so that the stop rule
//! can print it as it is. It names the file it could not read, or the model
//! or the effort it could not price.
//!
//! Definition only: the `Error` type.
//!
//! Spec 0079.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("nsx could not read the price list {}: {cause}.", path.display())]
	Prices {
		path: PathBuf,
		cause: crate::contract::error::Error,
	},
	#[error(
		"nsx could not read the price list {}: {model} has a negative price.",
		path.display()
	)]
	Negative { path: PathBuf, model: String },
	#[error(
		"nsx could not read the price list {}: {model} is not one of the three models nsx reads.",
		path.display()
	)]
	Beyond { path: PathBuf, model: String },
	#[error(
		"nsx could not read the price list {}: it lists {model} twice.",
		path.display()
	)]
	Twice { path: PathBuf, model: String },
	#[error("nsx could not read the forecast {}: {cause}.", path.display())]
	Forecasts {
		path: PathBuf,
		cause: crate::contract::error::Error,
	},
	#[error("The price list has no model {id}: nsx cannot price a run with it.")]
	Model { id: String },
	#[error(
		"The forecast has no expected tokens for the effort {effort}: nsx cannot price a run at it."
	)]
	Effort { effort: String },
	#[error(
		"The spend of {model} at the effort {effort} is too large to state in cents."
	)]
	Unpriced { model: String, effort: String },
	#[error("nsx could not write {} as JSON: {cause}.", path.display())]
	Unwritten {
		path: PathBuf,
		cause: serde_json::Error,
	},
	#[error(
		"The plan priced no run of {model} at the effort {effort}: plan the ticket again."
	)]
	Unplanned { model: String, effort: String },
	#[error(
		"The plan's spend table holds a figure nsx cannot use for {model} at the effort {effort}: plan the ticket again."
	)]
	Planned { model: String, effort: String },
	#[error(transparent)]
	File(#[from] crate::files::error::Error),
}
