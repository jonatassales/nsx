//! What reaching the console can refuse
//!
//! The errors of this folder, each one plain sentence, so that the stop rule
//! can print it as it is. None carries a value read from the secret, from
//! the credentials file or from the address a person gave: an address may
//! hold a password, and a file that does not read may hold the secret.
//!
//! Definition only: the `Error` type.
//!
//! Spec 0081.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(
		"nsx will not connect to the console's address: it is https, or http on this machine, with no user, path or query."
	)]
	Origin,
	#[error(
		"nsx will not record the secret: it must be printable text with no space."
	)]
	Secret,
	#[error(
		"nsx will not keep the console's connection in {}: Git tracks that file.",
		path.display()
	)]
	Tracked { path: PathBuf },
	#[error(
		"nsx will not read the console's connection from {}: others on this machine can read that file.",
		path.display()
	)]
	Exposed { path: PathBuf },
	#[error(
		"nsx could not read the console's connection in {}: it is not what nsx init recorded there.",
		path.display()
	)]
	Unread { path: PathBuf },
	#[error("nsx could not read the intent {}: {cause}.", path.display())]
	Intent {
		path: PathBuf,
		cause: crate::contract::error::Error,
	},
	#[error("nsx could not write {} as JSON: {cause}.", path.display())]
	Unwritten {
		path: PathBuf,
		cause: serde_json::Error,
	},
	#[error("nsx is not connected to a console.")]
	Offline,
	#[error("The console does not know the runtime nsx reported into.")]
	Unknown,
	#[error("The console is unavailable: {cause}.")]
	Unavailable { cause: String },
	#[error(transparent)]
	Program(#[from] crate::process::error::Error),
	#[error(transparent)]
	File(#[from] crate::files::error::Error),
}
