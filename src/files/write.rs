//! Writing a file whole or not at all
//!
//! One function: it takes a path and the bytes of a file, and replaces the
//! file at that path with them, so that a reader sees the old file or the new
//! one and never half of one.
//!
//! ## Reads and writes
//!
//! - writes the path it is given, under the project's root, through a
//!   draft beside it that is renamed over the file
//!
//! ## Rules
//!
//! - `plan.json` is replaced this way each time a line reveals a field, and
//!   `run.json` each time one of its sections changes, so that a receiver can
//!   follow the cards live (specs 0050 and 0065).
//! - A record that cannot be written is a stop, before the line that needed
//!   it.
//!
//! ## Open
//!
//! - `nsx init` keeps a file that exists byte for byte (spec 0048). Whether
//!   that is a second way of writing here or `command`'s own check is not
//!   settled.
//! - Spec 0073 asks for mode 0600 on the credentials file. Whether the mode
//!   is given to this function is not settled.
//!
//! Shared: used by every folder that writes a file under `.langdrift/`.

use std::path::Path;

use crate::files::error::Error;

pub fn write(path: &Path, bytes: &[u8]) -> Result<(), Error> {
	let unwritable = |cause: std::io::Error| Error::Unwritable {
		path: path.to_path_buf(),
		cause,
	};
	let nameless = || {
		unwritable(std::io::Error::new(
			std::io::ErrorKind::InvalidInput,
			"the path names no file",
		))
	};

	let folder = path.parent().ok_or_else(nameless)?;
	let name = path.file_name().ok_or_else(nameless)?;
	let draft = folder.join(format!(
		".{}.{}.draft",
		name.to_string_lossy(),
		std::process::id()
	));

	std::fs::create_dir_all(folder).map_err(unwritable)?;
	std::fs::write(&draft, bytes).map_err(unwritable)?;

	std::fs::rename(&draft, path).map_err(|cause| {
		// NOTE: the draft is ours to remove; the failure to report is the
		// rename's, whatever the removal does.
		let _left = std::fs::remove_file(&draft);

		unwritable(cause)
	})
}
