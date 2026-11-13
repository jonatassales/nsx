//! Whether a file is this person's alone
//!
//! One function: it takes a file and says whether nobody but its owner may
//! read or write it.
//!
//! ## Rules
//!
//! - On Unix it is the file's mode: no bit for the group or for others.
//! - Elsewhere the file's mode says nothing, and a regular file counts as
//!   private: the project's folder is the boundary there.

use std::path::Path;

use crate::files::error::Error;

pub fn private(file: &Path) -> Result<bool, Error> {
	let found = std::fs::metadata(file).map_err(|cause| Error::Unreadable {
		path: file.to_path_buf(),
		cause,
	})?;

	#[cfg(unix)]
	let alone = {
		use std::os::unix::fs::PermissionsExt;

		found.permissions().mode() & 0o077 == 0
	};

	#[cfg(not(unix))]
	let alone = found.is_file();

	Ok(alone)
}
