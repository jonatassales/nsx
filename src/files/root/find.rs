//! Finding the project's root
//!
//! One function: it takes the working directory and gives back the root: the
//! nearest folder, from there upward, that holds `.langdrift/`.
//!
//! ## Reads and writes
//!
//! - reads the working directory and its parents, looking for `.langdrift/`
//!
//! ## Rules
//!
//! - `nsx agent plan` and `nsx agent run` work from any folder inside the
//!   project.
//! - No root is a stop before any line: nothing is printed, one sentence goes
//!   to stderr and the command exits 1.
//!
//! Shared: used by `command`.

use std::path::Path;

use crate::files::{error::Error, root::Root};

impl Root {
	pub fn find(directory: &Path) -> Result<Self, Error> {
		let holder = directory
			.ancestors()
			.find(|folder| folder.join(".langdrift").is_dir());

		let Some(path) = holder else {
			return Err(Error::NoRoot {
				directory: directory.to_path_buf(),
			});
		};

		Ok(Self {
			path: path.to_path_buf(),
		})
	}
}
