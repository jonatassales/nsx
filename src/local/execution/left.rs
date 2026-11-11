//! What earlier runs left
//!
//! One function: it takes the project's root, the paths under `.langdrift/`
//! and a runtime's id, and gives back the execution ledgers of that
//! runtime's runs that are not settled, oldest first.
//!
//! ## Reads and writes
//!
//! - reads `.langdrift/agent/state/runs/<run>/execution.json`, of every run
//!   of the runtime
//!
//! ## Rules
//!
//! - A run of the runtime is a folder named `<runtime id>-<digits>`. Any
//!   other entry of the runs folder is not this runtime's, and is not read.
//! - A run with no ledger never reached its bring-up: it left nothing.
//! - A settled ledger left nothing: it is not looked at further.
//! - An unsettled ledger names the run of its folder and a space of that
//!   run's (`placed`). One that does not is a stop: nsx never deletes from
//!   a ledger it cannot place.
//!
//! ## Open
//!
//! - Nothing tells a run that is still going from one that stopped: the
//!   specs removed the locks. A second run of one runtime started in the
//!   same checkout while the first still has its runtime up would take that
//!   runtime down.
//!
//! Spec 0081.

use std::io::ErrorKind;

use crate::files::{inside::inside, layout::Layout, root::Root};
use crate::local::{
	error::Error,
	execution::{Execution, FILE},
};

impl Execution {
	pub fn left(
		root: &Root,
		layout: &Layout,
		runtime: &str,
	) -> Result<Vec<Self>, Error> {
		let folder = inside(root, &layout.runs)?;
		let unlisted = |cause: std::io::Error| Error::Unlisted {
			path: folder.clone(),
			cause,
		};
		let entries = match std::fs::read_dir(&folder) {
			Ok(entries) => entries,
			Err(cause) if cause.kind() == ErrorKind::NotFound => {
				return Ok(Vec::new());
			}
			Err(cause) => return Err(unlisted(cause)),
		};
		let prefix = format!("{runtime}-");
		let mut left = Vec::new();

		for entry in entries {
			let entry = entry.map_err(unlisted)?;
			let name = entry.file_name().to_string_lossy().into_owned();
			let numbered = name.strip_prefix(&prefix).is_some_and(|digits| {
				!digits.is_empty() && digits.chars().all(|digit| digit.is_ascii_digit())
			});
			let ours = numbered && entry.file_type().map_err(unlisted)?.is_dir();

			if !ours {
				continue;
			}

			let file = entry.path().join(FILE);
			let Some(execution) = Self::load(root, &file)? else {
				continue;
			};

			if execution.settled {
				continue;
			}

			if !execution.placed(root, &name, runtime) {
				return Err(Error::Misplaced {
					path: file,
				});
			}

			left.push(execution);
		}

		left.sort_by(|earlier, later| earlier.run.cmp(&later.run));

		Ok(left)
	}
}
