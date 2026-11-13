//! Cleaning up what a ledger records
//!
//! One function: it takes an execution ledger that is not settled, removes
//! what it records as created and settles it once the space's absence is
//! confirmed.
//!
//! ## Reads and writes
//!
//! - writes `.langdrift/agent/state/runs/<run>/execution.json`, as each
//!   Deployment is removed and once settled
//! - calls the cluster, to delete and to confirm absence
//!
//! ## Rules
//!
//! - It removes exactly what the ledger records, and nothing else: each
//!   Deployment by its name, then the run's space.
//! - A Deployment already recorded as removed is not asked for again, and a
//!   settled ledger asks nothing of the cluster.
//! - Everything is tried: a Deployment that would not go does not keep the
//!   others, or the space, from being asked for. Closing the space removes
//!   what is in it, so a space confirmed absent settles the ledger whatever
//!   was refused on the way.
//! - A space whose absence is not confirmed stays in the ledger, unsettled:
//!   the stopped line does not print and the run stops, saying each refusal
//!   it met, once. A space the cluster still lists is said to be left; one
//!   the cluster could not be asked about is said to be unconfirmed, never
//!   left. A later run of the same runtime finds the ledger and cleans it up
//!   (`left`).
//!
//! ## Open
//!
//! - Deleting what earlier stopped runs left, before a bring-up, is nsx's
//!   own choice: the simulation shows no such step.

use crate::files::{layout::Layout, root::Root};
use crate::local::{cluster::Cluster, error::Error, execution::Execution};

impl Execution {
	pub async fn cleanup(
		&mut self,
		root: &Root,
		layout: &Layout,
		cluster: &dyn Cluster,
	) -> Result<(), Error> {
		if self.settled {
			return Ok(());
		}

		let pending = self
			.deployments
			.iter()
			.filter(|name| !self.removed.contains(name))
			.cloned()
			.collect::<Vec<_>>();
		let mut refused = Vec::<String>::new();
		// Each refusal is kept once: one cause often refuses every step.
		let mut note = |refusal: Error| {
			let sentence = refusal.to_string();

			if !refused.contains(&sentence) {
				refused.push(sentence);
			}
		};

		for name in pending {
			match cluster.delete(&self.space, &name).await {
				Ok(()) => self.removed.push(name),
				Err(refusal) => note(refusal),
			}
		}

		if let Err(refusal) = cluster.close(&self.space).await {
			note(refusal);
		}

		let confirmed = match cluster.absent(&self.space).await {
			Ok(absent) => Some(absent),
			Err(refusal) => {
				note(refusal);

				None
			}
		};

		if confirmed == Some(true) {
			self.removed = self.deployments.clone();
			self.settled = true;

			return self.write(root, layout);
		}

		// What was removed is kept, so the next cleanup goes on from it.
		self.write(root, layout)?;

		let space = self.space.clone();
		let causes = refused
			.iter()
			.map(|refusal| format!(" {refusal}"))
			.collect();

		Err(match confirmed {
			Some(_present) => Error::Left {
				space,
				causes,
			},
			None => Error::Unconfirmed {
				space,
				causes,
			},
		})
	}
}
