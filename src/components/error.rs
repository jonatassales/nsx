//! What resolving the components can refuse
//!
//! The errors of this folder, each one plain sentence, so that the stop rule
//! can print it as it is. A sentence about one component names its path or
//! its file.
//!
//! Definition only: the `Error` type.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(
		"nsx found no folder {path} in the project: the spec names it as a component."
	)]
	Missing { path: String },
	#[error(
		"nsx found no manifest in {path}: a component holds next.config.ts, Cargo.toml, package.json, nats.conf or redis.conf."
	)]
	Unrecognised { path: String },
	#[error("nsx could not read {}: {cause}.", path.display())]
	Manifest {
		path: PathBuf,
		cause: serde_json::Error,
	},
	#[error(
		"nsx could not read the port {value:?} that {} declares: a port is a whole number from 1 to 65535.",
		path.display()
	)]
	Declared { path: PathBuf, value: String },
	#[error(
		"nsx found no port for {path}: neither its manifest, its stack nor the spec gives one."
	)]
	Port { path: String },
	#[error(
		"nsx has no share to propose for {path}: no share is set for {stack} · {role}."
	)]
	Share {
		path: String,
		stack: &'static str,
		role: &'static str,
	},
	#[error(
		"The image resolved for {path} is not an image reference: {image:?}."
	)]
	Image { path: String, image: String },
	#[error("The components collide: {collisions}.")]
	Collide { collisions: String },
	#[error(transparent)]
	File(#[from] crate::files::error::Error),
}
