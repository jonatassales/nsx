//! Recording what is about to be created
//!
//! One function: it takes the ledger and the name of one Deployment, and
//! writes the name to the run's execution ledger before the Deployment is
//! created.
//!
//! ## Reads and writes
//!
//! - writes `.langdrift/agent/state/runs/<run>/execution.json`
//!
//! ## Rules
//!
//! - The ledger is written first and the creation comes second, so cleanup
//!   acts only on what the run created.
//!
//! Spec 0081.

use crate::files::{layout::Layout, root::Root};
use crate::local::{error::Error, execution::Execution};

impl Execution {
	pub fn record(
		&mut self,
		root: &Root,
		layout: &Layout,
		deployment: &str,
	) -> Result<(), Error> {
		self.deployments.push(String::from(deployment));

		self.write(root, layout)
	}
}
