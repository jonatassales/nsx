//! Removing what earlier runs left
//!
//! One function: it takes the project's root, the paths under `.langdrift/`,
//! a runtime's id and the cluster, and cleans up every execution ledger of
//! that runtime that is not settled, oldest first.
//!
//! ## Reads and writes
//!
//! - reads and writes `.langdrift/agent/state/runs/<run>/execution.json`,
//!   of every unsettled run of the runtime
//! - calls the cluster, to delete and to confirm absence
//!
//! ## Rules
//!
//! - It runs before a run's first stage, for what earlier runs left. It
//!   prints nothing.
//! - A ledger that cannot be cleaned up is a stop: a run never brings its
//!   runtime up beside what an earlier one left.
//!
//! Spec 0081.

use crate::files::{layout::Layout, root::Root};
use crate::local::{cluster::Cluster, error::Error, execution::Execution};

impl Execution {
	pub async fn sweep(
		root: &Root,
		layout: &Layout,
		runtime: &str,
		cluster: &dyn Cluster,
	) -> Result<(), Error> {
		for mut left in Self::left(root, layout, runtime)? {
			left.cleanup(root, layout, cluster).await?;
		}

		Ok(())
	}
}
