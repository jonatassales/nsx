//! Confirming that a run's space is gone
//!
//! One function: it takes the space's name and says whether the cluster
//! holds no namespace of that run any more.
//!
//! ## Reads and writes
//!
//! - reaches the cluster (`reach`), then runs `kubectl get namespace`
//!
//! ## Rules
//!
//! - It asks the cluster and changes nothing in it. "Nothing left" is said
//!   from this answer and no other.
//! - A cluster that does not exist holds nothing of the run.

use crate::local::{
	error::Error,
	kind::{Kind, LABEL},
};

impl Kind<'_> {
	pub async fn absent(&self, space: &str) -> Result<bool, Error> {
		if !self.reach().await? {
			return Ok(true);
		}

		let labelled = format!("{LABEL}={space}");
		let listing = self.kubectl(
			&[
				"get",
				"namespace",
				"--selector",
				&labelled,
				"--output",
				"name",
			],
			Vec::new(),
		)?;
		let output = self.programs.finish(&listing).await?;
		let found = String::from_utf8_lossy(&output.printed);

		Ok(found.trim().is_empty())
	}
}
