//! Reading an execution ledger
//!
//! One function: it takes the project's root and a ledger's file, and gives
//! back the ledger, or nothing when the run left none.
//!
//! ## Reads and writes
//!
//! - reads `.langdrift/agent/state/runs/<run>/execution.json`
//!
//! ## Rules
//!
//! - Only a file that is not there means no ledger: a run that never reached
//!   its bring-up. One that is there and does not read is a stop.
//!
//! Spec 0081.

use std::io::ErrorKind;
use std::path::Path;

use crate::contract::decode::decode;
use crate::files::error::Error as Unread;
use crate::files::{
	root::Root,
	text::{DOCUMENT, text},
};
use crate::local::{error::Error, execution::Execution};

impl Execution {
	pub fn load(root: &Root, file: &Path) -> Result<Option<Self>, Error> {
		let written = match text(root, file, DOCUMENT) {
			Ok(written) => written,
			Err(Unread::Unreadable {
				cause,
				..
			}) if cause.kind() == ErrorKind::NotFound => return Ok(None),
			Err(refusal) => return Err(Error::File(refusal)),
		};
		let execution = decode(&written).map_err(|cause| Error::Unread {
			path: file.to_path_buf(),
			cause,
		})?;

		Ok(Some(execution))
	}
}
