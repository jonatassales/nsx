//! The port a manifest declares
//!
//! One function: it takes a manifest's path and its text, and gives back the
//! port the manifest declares, if it declares one.
//!
//! ## Rules
//!
//! - `nats.conf` and `redis.conf` declare it with a top-level key
//!   (`keyed.rs`), `package.json` in its `start` script (`scripted.rs`).
//! - `go.mod` cannot declare a port, and no rule says how `next.config.ts`
//!   would: neither declares one.
//! - A value taken from the environment (`port: $NATS_PORT`, `--port $PORT`)
//!   is not the manifest's: the manifest declares no port, and the next
//!   source is asked.
//! - Any other value that is not a whole number from 1 to 65535 stops the
//!   plan, with one sentence that names the file and the value. A port nsx
//!   cannot read is never replaced by another one.
//! - A `package.json` that is not JSON is refused.
//!
//! ## Open
//!
//! - How `next.config.ts` sets a port. Until a rule exists, a Next.js
//!   component serves on its stack's usual port.
//!
//! Spec 0079.

use std::path::Path;

use crate::components::{error::Error, manifest::Manifest};
use crate::contract::component::PORTS;

impl Manifest {
	pub fn port(&self, file: &Path, written: &str) -> Result<Option<i64>, Error> {
		let stated = match self {
			Self::NextConfig | Self::GoMod => None,
			Self::NatsConf | Self::RedisConf => self.keyed(written),
			Self::PackageJson => {
				Self::scripted(written).map_err(|cause| Error::Manifest {
					path: file.to_path_buf(),
					cause,
				})?
			}
		};
		let declared = stated.filter(|value| !value.starts_with('$'));
		let Some(value) = declared else {
			return Ok(None);
		};
		let port = value
			.parse::<i64>()
			.ok()
			.filter(|port| PORTS.contains(port));

		port.map(Some).ok_or_else(|| Error::Declared {
			path: file.to_path_buf(),
			value,
		})
	}
}
