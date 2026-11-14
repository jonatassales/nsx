//! A tool nsx keeps for the local cluster
//!
//! nsx brings its own kind and kubectl, so that a team installs nothing by
//! hand: one pinned version of each, fetched once into the project's private
//! state and checked against the SHA-256 its publisher gives.
//!
//! Definition only: the `Tool` type: the file's name, where it is published
//! and the SHA-256 it must have. The pinned versions sit beside it. Its
//! functions are the files in `tool/`.
//!
//! ## Rules
//!
//! - A tool is pinned to one version and one SHA-256 per machine: macOS and
//!   Linux, on arm64 and x64.
//! - Nothing that does not match its SHA-256 is ever written executable.
//! - The tools live in `.langdrift/agent/state/tools/`, private state Git
//!   ignores.
//! - Fetching prints no line: the simulation shows none.

pub mod fetch;
pub mod pinned;

pub const KIND_VERSION: &str = "v0.33.0";

pub const KUBECTL_VERSION: &str = "v1.37.2";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tool {
	pub name: &'static str,
	pub url: String,
	pub sha256: &'static str,
}
