//! Keeping a call's log
//!
//! One function: it takes the project's root and how the call ended, and
//! keeps everything the agent printed in the call's log.
//!
//! ## Reads and writes
//!
//! - writes `.langdrift/agent/state/runs/<run>/stages/<stage>.log`
//!
//! ## Rules
//!
//! - The log sits beside the report, private to its owner. It is never
//!   evidence.
//! - It is written after the usage record has kept the call: a log that
//!   cannot be written stops the run, and the paid call is still on record.

use crate::driver::{call::Call, ended::Ended, error::Error};
use crate::files::{inside::inside, root::Root, sealed::sealed};

impl Call {
	pub fn log(&self, root: &Root, ended: &Ended) -> Result<(), Error> {
		let name = format!("{}.log", self.stage);
		let log = inside(root, &self.report.with_file_name(name))?;

		Ok(sealed(&log, &ended.output)?)
	}
}
