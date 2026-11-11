//! Finding a component's manifest
//!
//! One function: it takes the project's root and a component's path, and
//! gives back the first manifest present in that folder, by a fixed
//! precedence.
//!
//! ## Reads and writes
//!
//! - reads the component's folder, in the client's project
//!
//! ## Rules
//!
//! - The precedence is `next.config.ts`, then `go.mod`, then `package.json`,
//!   then `nats.conf`, then `redis.conf`. The first file present decides.
//! - `next.config.ts` wins over a `package.json` beside it.
//! - A folder that is missing, or holds none of the five, stops the plan
//!   before the component's inspect line, under the stop rule.
//! - The folder is inside the project, and neither it nor its manifest is a
//!   symbolic link.
//!
//! Spec 0079.

use std::path::Path;

use crate::components::{
	error::Error,
	manifest::{MANIFESTS, Manifest},
};
use crate::files::{inside::inside, root::Root};

impl Manifest {
	pub fn find(root: &Root, path: &str) -> Result<Self, Error> {
		let folder = inside(root, Path::new(path))?;

		if !folder.is_dir() {
			return Err(Error::Missing {
				path: String::from(path),
			});
		}

		for manifest in MANIFESTS {
			let file = inside(root, &folder.join(manifest.file()))?;

			if file.is_file() {
				return Ok(manifest);
			}
		}

		Err(Error::Unrecognised {
			path: String::from(path),
		})
	}
}
