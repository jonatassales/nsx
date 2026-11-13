//! Giving a run its id
//!
//! One function: it takes the project's root, the paths under `.langdrift/`
//! and the runtime id, and gives back the run's id: the
//! highest run of that runtime in the money ledger plus one, as four digits.
//!
//! ## Reads and writes
//!
//! - reads the runs of the runtime in `.langdrift/finops/ledger/` and in
//!   `.langdrift/agent/state/runs/`
//!
//! ## Rules
//!
//! - The first run of a runtime is `<runtime id>-0001`.
//! - A run of this runtime is `<runtime id>-<digits>` and nothing between
//!   them. A number past 9999 keeps all its digits.
//! - In the ledger a run is a file, and among the private folders a folder:
//!   an entry of the other kind is no run.
//! - Both places a run leaves its name count: the money ledger, which the
//!   team commits, and the run's private folder, which Git ignores. So a
//!   checkout without an earlier run's ledger entry still does not reuse
//!   that run's id, nor the private files kept under it.
//! - The id is given when the run starts, before its first line.

use std::fs::FileType;
use std::path::Path;

use crate::evidence::{error::Error, id::Id};
use crate::files::{inside::inside, layout::Layout, root::Root};

impl Id {
	pub fn mint(
		root: &Root,
		layout: &Layout,
		runtime: &str,
	) -> Result<Self, Error> {
		// The highest number of this runtime among a folder's entries of one
		// kind, as `<runtime id>-<digits>` before the ending a place gives
		// its names.
		fn highest(
			folder: &Path,
			runtime: &str,
			ending: &str,
			kept: fn(&FileType) -> bool,
		) -> Result<u32, Error> {
			let unreadable = |cause: std::io::Error| {
				Error::File(crate::files::error::Error::Unreadable {
					path: folder.to_path_buf(),
					cause,
				})
			};
			let prefix = format!("{runtime}-");
			let mut found = 0_u32;

			if !folder.is_dir() {
				return Ok(found);
			}

			for entry in std::fs::read_dir(folder).map_err(unreadable)? {
				let entry = entry.map_err(unreadable)?;
				let kind = entry.file_type().map_err(unreadable)?;

				if !kept(&kind) {
					continue;
				}

				let name = entry.file_name();
				let digits = name
					.to_string_lossy()
					.strip_prefix(&prefix)
					.and_then(|rest| rest.strip_suffix(ending))
					.map(String::from);
				let numbered = |digits: &String| {
					digits.len() >= 4
						&& digits.chars().all(|digit| digit.is_ascii_digit())
				};
				let number = digits
					.filter(numbered)
					.and_then(|digits| digits.parse::<u32>().ok());

				found = found.max(number.unwrap_or(0));
			}

			Ok(found)
		}

		let ledger = inside(root, &layout.ledger)?;
		let runs = inside(root, &layout.runs)?;
		let recorded =
			highest(&ledger, runtime, ".ledger.json", FileType::is_file)?;
		let played = highest(&runs, runtime, "", FileType::is_dir)?;
		let number =
			recorded
				.max(played)
				.checked_add(1)
				.ok_or_else(|| Error::Exhausted {
					runtime: String::from(runtime),
				})?;

		Ok(Self {
			runtime: String::from(runtime),
			number,
		})
	}
}
