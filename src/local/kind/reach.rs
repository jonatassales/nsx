//! Reaching the shared cluster
//!
//! One function: it asks kind for its clusters and says whether the shared
//! one is among them. When it is, the project's kubeconfig is made to point
//! at it.
//!
//! ## Reads and writes
//!
//! - runs `kind get clusters`
//! - runs `kind export kubeconfig`, into the project's own kubeconfig
//!
//! ## Rules
//!
//! - The kubeconfig is written when the cluster is first reached: the
//!   cluster may have been created by another project, or made anew since
//!   the last run. Once reached, the cluster is not asked again.

use crate::local::{
	error::Error,
	kind::{CLUSTER, Kind, PASSED},
};
use crate::process::program::Program;

impl Kind<'_> {
	pub async fn reach(&self) -> Result<bool, Error> {
		if self.reached.get() {
			return Ok(true);
		}

		let listing =
			Program::tool("kind", &["get", "clusters"], &PASSED, &self.directory);
		let output = self.programs.finish(&listing).await?;
		let listed = String::from_utf8_lossy(&output.printed)
			.lines()
			.any(|name| name.trim() == CLUSTER);

		if !listed {
			return Ok(false);
		}

		let kubeconfig = self.kubeconfig()?;
		let exporting = Program::tool(
			"kind",
			&[
				"export",
				"kubeconfig",
				"--name",
				CLUSTER,
				"--kubeconfig",
				&kubeconfig,
			],
			&PASSED,
			&self.directory,
		);

		self.programs.finish(&exporting).await?;
		self.reached.set(true);

		Ok(true)
	}
}
