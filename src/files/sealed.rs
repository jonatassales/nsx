//! Writing a file only its owner may read
//!
//! One function: it takes a file and its bytes, and writes the file whole,
//! in a folder and with a mode that keep it from everybody but its owner.
//!
//! ## Rules
//!
//! - The folder is closed first (mode 0700), so the file is never readable by
//!   others while it is written, then the file is closed too (mode 0600).
//! - Where a file has no such mode, the file is written as any other.

use std::path::Path;

use crate::files::{error::Error, write::write};

pub fn sealed(file: &Path, bytes: &[u8]) -> Result<(), Error> {
	let unwritable = |cause: std::io::Error| Error::Unwritable {
		path: file.to_path_buf(),
		cause,
	};
	let folder = file.parent().unwrap_or(file);

	std::fs::create_dir_all(folder).map_err(unwritable)?;

	#[cfg(unix)]
	{
		use std::fs::Permissions;
		use std::os::unix::fs::PermissionsExt;

		std::fs::set_permissions(folder, Permissions::from_mode(0o700))
			.map_err(unwritable)?;
		write(file, bytes)?;
		std::fs::set_permissions(file, Permissions::from_mode(0o600))
			.map_err(unwritable)?;
	}

	#[cfg(not(unix))]
	write(file, bytes)?;

	Ok(())
}
