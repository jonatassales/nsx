//! Reading the usage record
//!
//! One function: it takes the project's root and the usage file, and gives
//! back the runs the file keeps: none when the runtime has no usage record
//! yet.
//!
//! ## Reads and writes
//!
//! - reads `.langdrift/microruntimes/runtimes/<runtime id>/usage.json`
//!
//! ## Rules
//!
//! - Only a file that is not there means no record yet. One that is there
//!   and does not read is a stop.
//! - A run asks before its first paid call: a usage record that does not
//!   read stops the run while nothing is owed, never after a call.

use std::io::ErrorKind;
use std::path::Path;

use crate::contract::decode::decode;
use crate::evidence::{error::Error, usage::Usage};
use crate::files::error::Error as Unread;
use crate::files::{
	inside::inside,
	root::Root,
	text::{DOCUMENT, text},
};

impl Usage {
	pub fn kept(root: &Root, file: &Path) -> Result<Vec<Self>, Error> {
		let target = inside(root, file)?;
		let written = match text(root, file, DOCUMENT) {
			Ok(written) => written,
			Err(Unread::Unreadable {
				cause,
				..
			}) if cause.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
			Err(refusal) => return Err(Error::File(refusal)),
		};

		decode(&written).map_err(|cause| Error::Unread {
			path: target,
			cause,
		})
	}
}
