//! Keeping a verified copy of a tool
//!
//! One function: it takes the folder nsx keeps its tools in and how to
//! download, and leaves there an executable copy of the tool whose SHA-256
//! is the pinned one.
//!
//! ## Reads and writes
//!
//! - reads the copy already in the folder, if any
//! - downloads the tool, through `Download`, only when no verified copy is
//!   there
//! - writes the tool, mode 0755, in the folder
//!
//! ## Rules
//!
//! - A copy whose SHA-256 is the pinned one is kept: nothing is downloaded.
//! - A copy that differs is replaced.
//! - Downloaded bytes that differ from the pin are refused and nothing is
//!   written.
//! - The bytes are written beside the tool and moved onto it, so a tool is
//!   never half written.

use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use sha2::{Digest, Sha256};

use crate::local::{download::Download, error::Error, tool::Tool};

impl Tool {
	pub async fn fetch(
		&self,
		folder: &Path,
		downloads: &dyn Download,
	) -> Result<(), Error> {
		let pinned = |bytes: &[u8]| {
			let digest = Sha256::digest(bytes);
			let spelled: String =
				digest.iter().map(|byte| format!("{byte:02x}")).collect();

			spelled == self.sha256
		};
		let unkept = |cause: std::io::Error| Error::Unkept {
			tool: self.name,
			cause,
		};
		let kept = folder.join(self.name);
		let verified = std::fs::read(&kept).is_ok_and(|bytes| pinned(&bytes));

		if verified {
			return Ok(());
		}

		let bytes =
			downloads
				.get(&self.url)
				.await
				.map_err(|cause| Error::Unfetched {
					tool: self.name,
					cause,
				})?;

		if !pinned(&bytes) {
			return Err(Error::Unpinned {
				tool: self.name,
			});
		}

		let arriving = folder.join(format!(".{}.arriving", self.name));

		std::fs::create_dir_all(folder).map_err(unkept)?;
		std::fs::write(&arriving, &bytes).map_err(unkept)?;
		std::fs::set_permissions(&arriving, std::fs::Permissions::from_mode(0o755))
			.map_err(unkept)?;
		std::fs::rename(&arriving, &kept).map_err(unkept)?;

		Ok(())
	}
}
