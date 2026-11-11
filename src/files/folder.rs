//! The folder a path names
//!
//! One function: it takes the project's root and a path, and gives back the
//! folder the disk names for it, so that two spellings of one folder give
//! one answer.
//!
//! ## Reads and writes
//!
//! - asks the disk for the folder's own name
//!
//! ## Rules
//!
//! - The path is checked as `inside.rs` checks every path: it never leaves
//!   the project and never passes through a symbolic link.
//! - `./services/api`, `services/api/` and, on a disk that ignores case,
//!   `services/API` give the same folder.
//! - A folder the disk cannot name is refused, with its path.
//!
//! Spec 0077.

use std::path::{Path, PathBuf};

use crate::files::{error::Error, inside::inside, root::Root};

pub fn folder(root: &Root, path: &Path) -> Result<PathBuf, Error> {
	let inside = inside(root, path)?;

	std::fs::canonicalize(&inside).map_err(|cause| Error::Unreadable {
		path: inside,
		cause,
	})
}
