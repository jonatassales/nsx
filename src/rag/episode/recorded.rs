//! The records in the episodes folder
//!
//! One function: it takes the project's root and the paths under
//! `.langdrift/`, and gives back the names of the JSON files in the
//! episodes folder, in order.
//!
//! ## Reads and writes
//!
//! - lists `.langdrift/microruntimes/episodes/`
//!
//! ## Rules
//!
//! - A project with no episodes folder has no past run: no name.
//! - Every `.json` file is listed, whatever its name: the index is built
//!   from all of them, so being in step with it is asked of all of them.
//! - The files are not read.

use std::io::ErrorKind;

use crate::files::{inside::inside, layout::Layout, root::Root};
use crate::rag::{episode::Episode, error::Error};

impl Episode {
	pub fn recorded(root: &Root, layout: &Layout) -> Result<Vec<String>, Error> {
		let folder = inside(root, &layout.episodes)?;
		let unsearchable = |cause: std::io::Error| Error::Unsearchable {
			path: folder.clone(),
			cause,
		};
		let entries = match std::fs::read_dir(&folder) {
			Ok(entries) => entries,
			Err(cause) if cause.kind() == ErrorKind::NotFound => {
				return Ok(Vec::new());
			}
			Err(cause) => return Err(unsearchable(cause)),
		};
		let mut names = Vec::new();

		for entry in entries {
			let name = entry.map_err(unsearchable)?.file_name();
			let name = name.to_string_lossy().into_owned();

			if name.ends_with(".json") {
				names.push(name);
			}
		}

		names.sort();

		Ok(names)
	}
}
