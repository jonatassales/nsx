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
//! - A copy whose SHA-256 is the pinned one is kept and made executable:
//!   nothing is downloaded.
//! - A copy that differs is replaced.
//! - Downloaded bytes that differ from the pin are refused and nothing is
//!   written.
//! - The bytes are written to a new file of this process's own beside the
//!   tool, never through an existing file or link, and moved onto the tool,
//!   so a tool is never half written, even when two runs fetch at once.

use std::io::Write;
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
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
			let runnable = std::fs::Permissions::from_mode(0o755);

			std::fs::set_permissions(&kept, runnable).map_err(unkept)?;

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

		let arriving =
			folder.join(format!(".{}.{}.arriving", self.name, std::process::id()));

		std::fs::create_dir_all(folder).map_err(unkept)?;

		// A file left by a process that had this id is not this one's: it is
		// removed, and the new one is created, never opened through a link.
		match std::fs::remove_file(&arriving) {
			Err(cause) if cause.kind() != std::io::ErrorKind::NotFound => {
				return Err(unkept(cause));
			}
			_removed => {}
		}

		let mut written = std::fs::OpenOptions::new()
			.write(true)
			.create_new(true)
			.mode(0o755)
			.open(&arriving)
			.map_err(unkept)?;

		written.write_all(&bytes).map_err(unkept)?;
		written.sync_all().map_err(unkept)?;
		std::fs::rename(&arriving, &kept).map_err(unkept)?;

		Ok(())
	}
}
