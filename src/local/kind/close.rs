//! Closing a run's space
//!
//! One function: it takes the space's name and deletes the run's namespace,
//! waiting until it is gone.
//!
//! ## Reads and writes
//!
//! - reaches the cluster (`reach`), then runs `kubectl delete namespace`
//!
//! ## Rules
//!
//! - The namespace is chosen by the run's label, never by its name alone:
//!   one that nsx did not create for this run is not deleted.
//! - A cluster that does not exist holds no namespace to delete.
//!
//! Spec 0081.

use crate::local::{
	error::Error,
	kind::{Kind, LABEL},
};

impl Kind<'_> {
	pub async fn close(&self, space: &str) -> Result<(), Error> {
		if !self.reach().await? {
			return Ok(());
		}

		let labelled = format!("{LABEL}={space}");
		let deleting = self.kubectl(
			&[
				"delete",
				"namespace",
				"--selector",
				&labelled,
				"--ignore-not-found",
				"--wait=true",
			],
			Vec::new(),
		)?;

		self.programs.finish(&deleting).await?;

		Ok(())
	}
}
