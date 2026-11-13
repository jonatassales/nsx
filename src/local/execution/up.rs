//! Bringing the runtime up
//!
//! One function: it takes the run's ledger, the cluster and the Deployments
//! of the applied `runtime.md` revision, and brings them up in the run's
//! space. It gives back nothing once every one is available, or the failure.
//!
//! ## Reads and writes
//!
//! - writes `.langdrift/agent/state/runs/<run>/execution.json`
//! - calls the cluster, to open the space and to apply each Deployment
//!
//! ## Rules
//!
//! - It runs after the `codeReview` line and before the `completed` line,
//!   never earlier.
//! - The steps, in order: open the run's space, which the ledger already
//!   records, then record and apply each Deployment. Applying waits until
//!   the Deployment reports itself available.
//! - Nothing is built, and no line prints until the ready line.
//! - A space not opened, an image not available or a Deployment never
//!   available is a stop under the stop rule: what was created is taken
//!   down and the ledger is settled (`cleanup`).

use crate::files::{layout::Layout, root::Root};
use crate::local::{
	cluster::Cluster, deployment::Deployment, error::Error, execution::Execution,
};

impl Execution {
	pub async fn up(
		&mut self,
		root: &Root,
		layout: &Layout,
		cluster: &dyn Cluster,
		deployments: &[Deployment],
	) -> Result<(), Error> {
		cluster.open(&self.space).await?;

		for deployment in deployments {
			self.record(root, layout, &deployment.name)?;

			cluster.apply(&self.space, deployment).await?;
		}

		Ok(())
	}
}
