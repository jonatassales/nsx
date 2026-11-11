//! Recording the team's authorization
//!
//! One function: it takes the project's root, the intent's file and the
//! proposed intent the team's Play matched, writes `status: authorized` into
//! `intent.json` and gives back the authorized intent.
//!
//! ## Reads and writes
//!
//! - writes `.langdrift/microruntimes/runtimes/<runtime id>/intent.json`
//!
//! ## Rules
//!
//! - nsx records the team's authorization, it never decides it: this runs
//!   only after a Play whose choice equals the intent's.
//! - Nothing but the status changes in the file.
//!
//! Spec 0079.

use std::path::Path;

use crate::contract::{encode::encode, intent::Intent};
use crate::files::{inside::inside, root::Root, write::write};
use crate::handoff::{authorization::Authorization, error::Error};

const AUTHORIZED: &str = "authorized";

impl Authorization {
	pub fn grant(
		root: &Root,
		file: &Path,
		proposed: Intent,
	) -> Result<Intent, Error> {
		let target = inside(root, file)?;
		let intent = Intent {
			status: String::from(AUTHORIZED),
			..proposed
		};
		let mut written = encode(&intent).map_err(|cause| Error::Unwritten {
			path: target.clone(),
			cause,
		})?;

		written.push('\n');
		write(&target, written.as_bytes())?;

		Ok(intent)
	}
}
