//! Applying a Deployment
//!
//! One function: it takes the run's space and a Deployment, makes the image
//! available to the cluster, applies the Deployment and waits until it
//! reports itself available.
//!
//! ## Reads and writes
//!
//! - runs `docker image inspect`, and `kind load docker-image` for an image
//!   that is on this machine
//! - runs `kubectl apply`, with the Deployment on its standard input, then
//!   `kubectl rollout status`
//!
//! ## Rules
//!
//! - An image on this machine is loaded into the cluster. Any other is left
//!   for the cluster to pull. Nothing is built.
//! - "Available" is the Deployment's own condition: no probe is added.
//! - The wait is bounded (`WAIT`), and the Deployment's own progress
//!   deadline is set to the same bound. A Deployment that is not available
//!   by then is a stop.
//!
//! ## Open
//!
//! - An image is to be loaded only when this machine alone holds it. nsx
//!   cannot tell that without pulling, so an image this machine holds is
//!   always loaded.

use crate::local::{
	deployment::Deployment,
	error::Error,
	kind::{CLUSTER, Kind, PASSED, WAIT},
};
use crate::process::program::Program;

impl Kind<'_> {
	pub async fn apply(
		&self,
		space: &str,
		deployment: &Deployment,
	) -> Result<(), Error> {
		let image = deployment.image.as_str();
		let inspecting = Program::tool(
			"docker",
			&["image", "inspect", "--", image],
			&PASSED,
			&self.directory,
		);
		// Docker ends with another status for an image it does not hold:
		// that is an answer, not a failure.
		let local = self.programs.run(&inspecting).await?.status == Some(0);

		if local {
			let loading = Program::tool(
				"kind",
				&["load", "docker-image", "--name", CLUSTER, "--", image],
				&PASSED,
				&self.directory,
			);

			self.programs.finish(&loading).await?;
		}

		let manifest = deployment.manifest(space).to_string().into_bytes();
		let applying = self.kubectl(&["apply", "-f", "-"], manifest)?;

		self.programs.finish(&applying).await?;

		let named = format!("deployment/{}", deployment.name);
		let bound = format!("{WAIT}s");
		let waiting = self.kubectl(
			&[
				"rollout",
				"status",
				&named,
				"--namespace",
				space,
				"--timeout",
				&bound,
			],
			Vec::new(),
		)?;

		self.programs.finish(&waiting).await?;

		Ok(())
	}
}
