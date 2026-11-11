//! What can go wrong with the project's files
//!
//! Each failure of this folder, worded as the one plain sentence the stop
//! rule sends to standard error.
//!
//! Definition only: the `Error` type, with one failure per way a file of the
//! project can refuse nsx.
//!
//! ## Rules
//!
//! - A sentence names the path and the cause, and never a file's content.
//!
//! Shared: used by every folder that reads or writes a file of the project.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(
		"nsx found no project: no folder from {} upward holds .langdrift/, which nsx init creates.",
		directory.display()
	)]
	NoRoot { directory: PathBuf },
	#[error("nsx could not read {}: {cause}.", path.display())]
	Unreadable {
		path: PathBuf,
		cause: std::io::Error,
	},
	#[error(
		"nsx will not read {}: it is larger than {limit} bytes.",
		path.display()
	)]
	TooLarge { path: PathBuf, limit: u64 },
	#[error("nsx could not write {}: {cause}.", path.display())]
	Unwritable {
		path: PathBuf,
		cause: std::io::Error,
	},
	#[error("nsx will not follow {}: it leaves the project.", path.display())]
	Outside { path: PathBuf },
	#[error(
		"nsx will not follow {}: it is a symbolic link.",
		path.display()
	)]
	Linked { path: PathBuf },
	#[error("nsx will not read {}: it is not a regular file.", path.display())]
	NotFile { path: PathBuf },
	#[error(
		"nsx will not read {}: it is not UTF-8 text ({cause}).",
		path.display()
	)]
	NotText {
		path: PathBuf,
		cause: std::string::FromUtf8Error,
	},
}
