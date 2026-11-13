//! One component of the client's project
//!
//! A component is a folder in the client's project, named by its path as the
//! spec writes it. The plan resolves the rest: the manifest that decided the
//! stack, the stack and the role, the port it serves on, the image it runs and
//! the share proposed for it.
//!
//! Definition only: the `Component` type: its path as the spec writes it,
//! the folder that path names on the disk, its manifest, stack, role, port,
//! image and proposed share, in millicores and MiB. The image is absent
//! until the plan resolves it, after the last component's row. Its functions
//! are the files in `component/`.
//!
//! ## Rules
//!
//! - The path is the component's identity: components are unique by path.
//!   Two paths are the same component when they name the same folder,
//!   however each is spelled.
//! - The share comes from a fixed table by stack and role: Rust api 250m and
//!   256 MiB, Node.js api 300m and 512 MiB, NATS broker 200m and 256 MiB,
//!   Redis cache 200m and 256 MiB, Next.js web 500m and 1024 MiB.
//! - The share is whole millicores and whole MiB. The port is an integer.
//!
//! ## Open
//!
//! - Where the shares come from. The fixed table is a choice: the simulation
//!   shows the shares, not how a share is derived.

pub mod card;
pub mod inspect;
pub mod line;
pub mod row;

use std::path::PathBuf;

use crate::components::{
	image::Image, manifest::Manifest, role::Role, stack::Stack,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Component {
	pub path: String,
	pub folder: PathBuf,
	pub manifest: Manifest,
	pub stack: Stack,
	pub role: Role,
	pub port: i64,
	pub image: Option<Image>,
	pub cpu_millis: i64,
	pub memory_mib: i64,
}
