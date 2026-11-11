//! Writing the execution ledger
//!
//! One function: it takes the ledger, the project's root and the paths under
//! `.langdrift/`, and writes the ledger to its run's private folder.
//!
//! ## Reads and writes
//!
//! - writes `.langdrift/agent/state/runs/<run>/execution.json`
//!
//! ## Rules
//!
//! - The file is private: only the person who runs nsx reads it.
//!
//! Spec 0081.

use crate::contract::encode::encode;
use crate::files::{
	inside::inside, layout::Layout, root::Root, sealed::sealed,
};
use crate::local::{
	error::Error,
	execution::{Execution, FILE},
};

impl Execution {
	pub fn write(&self, root: &Root, layout: &Layout) -> Result<(), Error> {
		let file = inside(root, &layout.runs.join(&self.run).join(FILE))?;
		let mut written = encode(self).map_err(|cause| Error::Unwritten {
			path: file.clone(),
			cause,
		})?;

		written.push('\n');

		Ok(sealed(&file, written.as_bytes())?)
	}
}
