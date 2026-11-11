//! Deleting a Deployment
//!
//! One function: it takes the run's space and a Deployment's name, and
//! deletes the Deployment, waiting until it is gone.
//!
//! ## Reads and writes
//!
//! - reaches the cluster (`reach`), then runs `kubectl delete deployment`
//!
//! ## Rules
//!
//! - A Deployment that is not there is already deleted, and so is any
//!   Deployment of a cluster that does not exist.
//!
//! Spec 0081.

use crate::local::{error::Error, kind::Kind};

impl Kind<'_> {
	pub async fn delete(&self, space: &str, name: &str) -> Result<(), Error> {
		if !self.reach().await? {
			return Ok(());
		}

		let deleting = self.kubectl(
			&[
				"delete",
				"deployment",
				name,
				"--namespace",
				space,
				"--ignore-not-found",
				"--wait=true",
			],
			Vec::new(),
		)?;

		self.programs.finish(&deleting).await?;

		Ok(())
	}
}
