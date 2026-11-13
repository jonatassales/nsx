//! Finding the ticket from its id
//!
//! One function: it takes the bare id a person typed and gives back the
//! project-relative path of the one file named `<id>.ticket.md`.
//!
//! ## Reads and writes
//!
//! - reads the client's own artifact folder, `.agents/` or `.claude/` at the
//!   project root, never `.langdrift/`
//!
//! ## Rules
//!
//! - No ticket for the id, or more than one, stops the plan before the line
//!   this folder would print next, with no draft written (the stop rule). A
//!   plan never runs on a guess.
//!
//! - Both folders are searched, at any depth, and the id must name one file
//!   across the two.
//! - The id is matched against file names only. It is never joined to a
//!   path, so an id that climbs finds nothing.
//!
//! ## Open
//!
//! - The client's artifact folder is `.agents/` or `.claude/`. Which one it
//!   is when a project has both is not settled. Until the owner decides,
//!   both are searched.

use std::ffi::OsString;
use std::io::ErrorKind;
use std::path::PathBuf;

use crate::files::root::Root;
use crate::request::{error::Error, ticket::Ticket};

impl Ticket {
	pub fn resolve(root: &Root, id: &str) -> Result<PathBuf, Error> {
		let name = OsString::from(format!("{id}.ticket.md"));
		let mut folders = vec![PathBuf::from(".agents"), PathBuf::from(".claude")];
		let mut found = Vec::new();

		// A folder that is a symbolic link is not a folder here, so the search
		// never leaves the project.
		while let Some(folder) = folders.pop() {
			let unsearchable = |cause: std::io::Error| Error::Unsearchable {
				path: folder.clone(),
				cause,
			};

			let entries = match std::fs::read_dir(root.path.join(&folder)) {
				Ok(entries) => entries,
				Err(cause) if cause.kind() == ErrorKind::NotFound => continue,
				Err(cause) => return Err(unsearchable(cause)),
			};

			for entry in entries {
				let entry = entry.map_err(unsearchable)?;
				let path = folder.join(entry.file_name());

				if entry.file_type().map_err(unsearchable)?.is_dir() {
					folders.push(path);

					continue;
				}

				if entry.file_name() == name {
					found.push(path);
				}
			}
		}

		let mut found = found.into_iter();

		match (found.next(), found.next()) {
			(Some(path), None) => Ok(path),
			(None, _) => Err(Error::NoTicket {
				id: String::from(id),
			}),
			(Some(_first), Some(_second)) => Err(Error::Tickets {
				id: String::from(id),
			}),
		}
	}
}
