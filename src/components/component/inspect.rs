//! Inspecting a component
//!
//! One function: it takes the project's root and a path the spec names, with
//! the port the spec names beside it if any, and gives back the component:
//! its manifest, stack, role, port and proposed share. Its image is resolved
//! later, in `image/resolve.rs`.
//!
//! ## Reads and writes
//!
//! - reads the manifest in the component's folder, in the client's project
//! - asks the disk which folder the path names
//!
//! ## Rules
//!
//! - The port is the one the manifest declares: `port` in `nats.conf`, `port`
//!   in `redis.conf`, `PORT=`, `--port` or `-p` in the `start` script of a
//!   `package.json`. Otherwise it is the stack's usual port: NATS 4222, Redis
//!   6379, Next.js 3000. Otherwise it is the port the spec names beside the
//!   path.
//! - A port is a whole number from 1 to 65535. A manifest that declares
//!   anything else stops the plan (`manifest/port.rs`), and so does a
//!   component whose only source is a spec port outside that range.
//! - The share comes from the fixed table by stack and role.
//! - The folder is the one the disk gives for the path, so that two
//!   spellings of one folder are known to be one component.
//! - A path that is missing, has none of the five manifests or gets no port
//!   stops the plan before the component's inspect line: no further line, one
//!   sentence on stderr that names the path, no draft written, exit 1.
//! - No Dockerfile, healthcheck, env file or ceiling is read.
//!
//! ## Open
//!
//! - Where the port comes from when the manifest cannot declare one, as with
//!   `Cargo.toml`. The chain above is provisional: every component's port may
//!   come from the spec instead.
//!
//! Spec 0079.

use std::path::Path;

use crate::components::{
	component::Component,
	error::Error,
	manifest::Manifest,
	role::{API, BROKER, CACHE, WEB},
	stack::{NATS, NEXT, NODE, REDIS, RUST},
};
use crate::contract::component::PORTS;
use crate::files::{
	folder::folder,
	root::Root,
	text::{DOCUMENT, text},
};
use crate::request::named::Named;

impl Component {
	pub fn inspect(root: &Root, named: &Named) -> Result<Self, Error> {
		let manifest = Manifest::find(root, &named.path)?;
		let folder = folder(root, Path::new(&named.path))?;
		let file = Path::new(&named.path).join(manifest.file());
		let written = text(root, &file, DOCUMENT)?;
		let declared = manifest.port(&file, &written)?;
		let (stack, role) = manifest.classify();
		let (cpu_millis, memory_mib) = match (stack, role) {
			(RUST, API) => (250, 256),
			(NODE, API) => (300, 512),
			(NATS, BROKER) => (200, 256),
			(REDIS, CACHE) => (200, 256),
			(NEXT, WEB) => (500, 1024),
			_ => {
				return Err(Error::Share {
					path: named.path.clone(),
					stack: stack.name,
					role: role.name,
				});
			}
		};
		let named_port = named.port.filter(|port| PORTS.contains(port));
		let port =
			declared
				.or(stack.port)
				.or(named_port)
				.ok_or_else(|| Error::Port {
					path: named.path.clone(),
				})?;

		Ok(Self {
			path: named.path.clone(),
			folder,
			manifest,
			stack,
			role,
			port,
			image: None,
			cpu_millis,
			memory_mib,
		})
	}
}
