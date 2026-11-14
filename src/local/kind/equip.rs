//! Bringing nsx's own tools
//!
//! One function: before the cluster is first reached, it leaves nsx's pinned
//! kind and kubectl, verified, in the project's tools folder.
//!
//! ## Reads and writes
//!
//! - downloads and writes the pinned tools, through `tool/fetch.rs`, into
//!   `.langdrift/agent/state/tools/`
//!
//! ## Rules
//!
//! - It runs once for each `Kind`.
//! - It prints nothing. A tool that cannot be fetched or does not match its
//!   pin is a stop under the stop rule.

use crate::local::{error::Error, kind::Kind};

impl Kind<'_> {
	pub async fn equip(&self) -> Result<(), Error> {
		if self.equipped.get() {
			return Ok(());
		}

		let folder = self.tools()?;

		for tool in &self.pinned {
			tool.fetch(&folder, self.downloads).await?;
		}

		self.equipped.set(true);

		Ok(())
	}
}
