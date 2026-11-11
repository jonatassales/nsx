//! Opening a runtime's reports
//!
//! One function: it takes the project's root, the paths under `.langdrift/`
//! and the runtime's id in the project, and gives back the report state a
//! command starts from, with the console's id of the runtime when an earlier
//! command kept it.
//!
//! ## Reads and writes
//!
//! - reads `.langdrift/agent/state/console/<runtime id>.id`
//!
//! ## Rules
//!
//! - A runtime with no kept id sends none: the console opens a new runtime
//!   and answers its id.
//!
//! Spec 0081.

use crate::console::report::Report;
use crate::contract::status::Status;
use crate::files::{
	error::Error,
	inside::inside,
	layout::Layout,
	root::Root,
	text::{DOCUMENT, text},
};

impl Report {
	pub fn open(
		root: &Root,
		layout: &Layout,
		runtime: &str,
		status: Status,
	) -> Result<Self, Error> {
		let kept = inside(root, &layout.reported.join(format!("{runtime}.id")))?;
		let known = if kept.exists() {
			Some(text(root, &kept, DOCUMENT)?.trim().to_string())
		} else {
			None
		};

		Ok(Self {
			kept,
			runtime: known.filter(|id| !id.is_empty()),
			status,
			unavailable: false,
		})
	}
}
