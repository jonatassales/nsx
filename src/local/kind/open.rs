//! Opening a run's space
//!
//! One function: it takes the space's name, creates the shared cluster when
//! the machine has none, and creates the run's namespace in it.
//!
//! ## Reads and writes
//!
//! - reaches the cluster (`reach`), and runs `kind create cluster` when the
//!   shared cluster is not there
//! - runs `kubectl apply`, with the namespace on its standard input
//!
//! ## Rules
//!
//! - A cluster created here writes its access into the project's own
//!   kubeconfig, never the person's.
//! - The namespace is named after the space and carries its label.
//!
//! Spec 0081.

use serde_json::json;

use crate::local::{
	error::Error,
	kind::{CLUSTER, Kind, LABEL, PASSED},
};
use crate::process::program::Program;

impl Kind<'_> {
	pub async fn open(&self, space: &str) -> Result<(), Error> {
		if !self.reach().await? {
			let kubeconfig = self.kubeconfig()?;
			let creating = Program::tool(
				"kind",
				&[
					"create",
					"cluster",
					"--name",
					CLUSTER,
					"--kubeconfig",
					&kubeconfig,
					"--wait",
					"120s",
				],
				&PASSED,
				&self.directory,
			);

			self.programs.finish(&creating).await?;
			self.reached.set(true);
		}

		let namespace = json!({
			"apiVersion": "v1",
			"kind": "Namespace",
			"metadata": {
				"name": space,
				"labels": {
					LABEL: space,
				},
			},
		});
		let applying = self
			.kubectl(&["apply", "-f", "-"], namespace.to_string().into_bytes())?;

		self.programs.finish(&applying).await?;

		Ok(())
	}
}
