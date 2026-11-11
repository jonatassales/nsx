//! Reading a file up to a limit
//!
//! One function: it takes a path and a limit in bytes, and gives back the
//! file's bytes, or refuses the file.
//!
//! ## Reads and writes
//!
//! - reads the path it is given
//!
//! ## Rules
//!
//! - A file that is missing or cannot be read is a stop for its caller: a
//!   linked document that is unreadable stops the plan (spec 0053).
//! - Only a regular file is read. A pipe, a device or a folder is refused
//!   before a byte is asked of it.
//!
//! ## Open
//!
//! - No spec sets a limit on a file nsx reads, or says what a larger file
//!   does. Spec 0053 removes the old limits on what a plan collects. Which
//!   files are bounded, and at how many bytes, is to decide.
//!
//! Shared: used by every folder that reads a file of the project.

use std::path::Path;

use crate::files::error::Error;

pub fn read(path: &Path, limit: u64) -> Result<Vec<u8>, Error> {
	let unreadable = |cause: std::io::Error| Error::Unreadable {
		path: path.to_path_buf(),
		cause,
	};

	let found = std::fs::metadata(path).map_err(unreadable)?;

	// A pipe or a device never ends, and a folder has no bytes to read.
	if !found.is_file() {
		return Err(Error::NotFile {
			path: path.to_path_buf(),
		});
	}

	if found.len() > limit {
		return Err(Error::TooLarge {
			path: path.to_path_buf(),
			limit,
		});
	}

	std::fs::read(path).map_err(unreadable)
}
