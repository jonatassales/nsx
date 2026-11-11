//! Opening the run's execution ledger
//!
//! One function: it takes the project's root, the paths under `.langdrift/`,
//! the run's id and its runtime's, and gives back the ledger, written, with
//! the run's space recorded and nothing else.
//!
//! ## Reads and writes
//!
//! - writes `.langdrift/agent/state/runs/<run>/execution.json`
//!
//! ## Rules
//!
//! - The space is named after the run and this project (`space`), and it is
//!   written down here, before the cluster is asked for it.
//!
//! Spec 0081.

use crate::files::{layout::Layout, root::Root};
use crate::local::{error::Error, execution::Execution};

impl Execution {
	pub fn open(
		root: &Root,
		layout: &Layout,
		run: &str,
		runtime: &str,
	) -> Result<Self, Error> {
		let execution = Self {
			run: String::from(run),
			runtime: String::from(runtime),
			space: Self::space(root, run),
			deployments: Vec::new(),
			removed: Vec::new(),
			settled: false,
		};

		execution.write(root, layout)?;

		Ok(execution)
	}
}
