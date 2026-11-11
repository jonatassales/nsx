//! The revision of the plan being made
//!
//! One function: it takes the project's root and the runtime's file, and
//! gives back the revision the new draft takes: 1 for a runtime's first
//! plan, the previous draft's revision plus one after that.
//!
//! ## Reads and writes
//!
//! - reads `.langdrift/microruntimes/runtimes/<runtime id>/runtime.md`, when
//!   an earlier plan left one
//!
//! ## Rules
//!
//! - The previous revision is read from the earlier draft's `langdrift` block
//!   alone, whatever its version: the rest of the block is not checked.
//! - An earlier draft with no revision this function can read gives
//!   revision 1.
//! - An earlier draft nsx cannot read as a file stops the plan: it is never
//!   written over unread.
//!
//! Spec 0079.

use std::path::Path;

use serde_json::Value;

use crate::contract::fence::{CLOSING, OPENING};
use crate::files::{
	error::Error,
	inside::inside,
	root::Root,
	text::{DOCUMENT, text},
};
use crate::runtime::revision::Revision;

impl Revision {
	pub fn next(root: &Root, file: &Path) -> Result<Self, Error> {
		let earlier = inside(root, file)?;

		if !earlier.exists() {
			return Ok(Self(1));
		}

		let written = text(root, file, DOCUMENT)?;
		let block = written
			.split_once(OPENING)
			.and_then(|(_heading, after)| after.split_once(CLOSING))
			.map(|(block, _rest)| block);
		let previous = block
			.and_then(|block| serde_json::from_str::<Value>(block).ok())
			.and_then(|fence| fence.get("revision").and_then(Value::as_i64))
			.filter(|revision| *revision >= 1);
		let next = previous
			.and_then(|revision| revision.checked_add(1))
			.unwrap_or(1);

		Ok(Self(next))
	}
}
