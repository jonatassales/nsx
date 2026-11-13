//! Choosing the intent a run applies
//!
//! One function: it takes the project's root and the paths under
//! `.langdrift/`, reads every recorded intent and gives back the one a run
//! applies: the one the team authorized whose run has not completed or,
//! when there is none, the one intent still proposed, which a connected run
//! waits on (`console/play`).
//!
//! ## Reads and writes
//!
//! - reads `.langdrift/microruntimes/runtimes/<runtime id>/intent.json` of
//!   every runtime, and its `run.json` when there is one
//!
//! ## Rules
//!
//! - `nsx agent run` takes no argument: the team's authorization chooses.
//! - An intent whose `run.json` says `completed` has been applied and is
//!   not chosen again.
//! - No authorized intent and no single proposed one is a stop. A proposed
//!   intent is given back as it is: the run says the team has not
//!   authorized it yet, or waits for the team's Play when it is connected.
//! - Several authorized intents are a stop that names their tickets: nsx
//!   never picks between two things the team authorized.
//! - An intent or a run record that does not read is a stop, with its file:
//!   a folder or a broken link under that name is never skipped.
//! - An intent that names another runtime than the folder it sits in is a
//!   stop: its run would write into the other runtime's records.

use std::path::Path;

use serde::de::DeserializeOwned;

use crate::contract::{
	decode::decode, intent::Intent, outcome::Outcome, run::Run,
};
use crate::files::{
	inside::inside,
	layout::Layout,
	root::Root,
	text::{DOCUMENT, text},
};
use crate::handoff::{authorization::Authorization, error::Error};

impl Authorization {
	pub fn choose(root: &Root, layout: &Layout) -> Result<Intent, Error> {
		// A record of a runtime's folder, when the folder holds anything under
		// that name: a file that does not read is a stop, never a skip.
		fn recorded<Record: DeserializeOwned>(
			root: &Root,
			file: &Path,
		) -> Result<Option<Record>, Error> {
			if std::fs::symlink_metadata(file).is_err() {
				return Ok(None);
			}

			let written = text(root, file, DOCUMENT)?;
			let record = decode(&written).map_err(|cause| Error::Unread {
				path: file.to_path_buf(),
				cause,
			})?;

			Ok(Some(record))
		}

		let runtimes = inside(root, &layout.runtimes)?;
		let unreadable = |cause: std::io::Error| {
			Error::File(crate::files::error::Error::Unreadable {
				path: runtimes.clone(),
				cause,
			})
		};
		let mut folders = Vec::new();

		if runtimes.is_dir() {
			for entry in std::fs::read_dir(&runtimes).map_err(unreadable)? {
				folders.push(entry.map_err(unreadable)?.path());
			}
		}

		folders.sort();

		let mut authorized = Vec::new();
		let mut proposed = Vec::new();

		for folder in folders {
			let file = folder.join("intent.json");
			let Some(intent) = recorded::<Intent>(root, &file)? else {
				continue;
			};
			let ran = recorded::<Run>(root, &folder.join("run.json"))?;
			let placed = folder
				.file_name()
				.is_some_and(|name| name.to_string_lossy() == intent.runtime_id);
			let completed =
				ran.is_some_and(|run| run.outcome == Some(Outcome::Completed));
			let runnable = intent.status == "authorized" && !completed;

			if !placed {
				return Err(Error::Misplaced {
					path: file,
				});
			}

			if runnable {
				authorized.push(intent);
			} else if intent.status == "proposed" {
				proposed.push(intent);
			}
		}

		if authorized.len() > 1 {
			let tickets = authorized
				.iter()
				.map(|intent| intent.ticket.as_str())
				.collect::<Vec<_>>();

			return Err(Error::Several {
				tickets: tickets.join(", "),
			});
		}

		let waiting = proposed.len() == 1;

		match (authorized.pop(), proposed.pop()) {
			(Some(intent), _waiting) => Ok(intent),
			(None, Some(intent)) if waiting => Ok(intent),
			(None, _others) => Err(Error::Nothing),
		}
	}
}
