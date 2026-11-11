//! Reading a text
//!
//! How every document nsx reads comes in: the ticket, the documents it links,
//! the guardrail. A text is bounded and is UTF-8, or it is refused.
//!
//! One function: it takes the project's root, a path and a bound in bytes,
//! and gives back the file's text.
//!
//! ## Rules
//!
//! - The path is one `inside` accepts: it never leaves the project and never
//!   passes through a symbolic link.
//! - A text is read up to the bound its caller gives. A document is at most
//!   1 MiB, the bound `runtime.md` has.
//! - A file that is not UTF-8 is refused, never read lossily.
//!
//! Spec 0077.

use std::path::Path;

use crate::files::{error::Error, inside::inside, read::read, root::Root};

pub const DOCUMENT: u64 = 1 << 20;

pub fn text(root: &Root, path: &Path, limit: u64) -> Result<String, Error> {
	let inside = inside(root, path)?;
	let bytes = read(&inside, limit)?;

	String::from_utf8(bytes).map_err(|cause| Error::NotText {
		path: inside,
		cause,
	})
}
