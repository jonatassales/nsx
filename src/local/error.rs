//! What the local cluster can refuse
//!
//! The errors of this folder, each one plain sentence, so that the stop rule
//! can print it as it is.
//!
//! Definition only: the `Error` type.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(
		"nsx cannot name a Deployment after the component {path}: its path must make a lowercase label."
	)]
	Unnamed { path: String },
	#[error(
		"The draft is at revision {draft} and the run applies revision {applied}: nsx will not bring up another revision."
	)]
	Revised { applied: i64, draft: i64 },
	#[error("nsx could not write {} as JSON: {cause}.", path.display())]
	Unwritten {
		path: PathBuf,
		cause: serde_json::Error,
	},
	#[error("nsx could not read the execution ledger {}: {cause}.", path.display())]
	Unread {
		path: PathBuf,
		cause: crate::contract::error::Error,
	},
	#[error(
		"nsx will not clean up from the execution ledger {}: it does not name the run of its folder in this project.",
		path.display()
	)]
	Misplaced { path: PathBuf },
	#[error("nsx could not list the runs in {}: {cause}.", path.display())]
	Unlisted {
		path: PathBuf,
		cause: std::io::Error,
	},
	#[error(
		"The runtime's namespace {space} is still in the cluster: something of the run is left.{causes}"
	)]
	Left { space: String, causes: String },
	#[error(
		"nsx could not confirm that the runtime's namespace {space} is gone.{causes}"
	)]
	Unconfirmed { space: String, causes: String },
	#[error(
		"nsx cannot give kind and kubectl the path {}: it is not UTF-8.",
		path.display()
	)]
	Unspoken { path: PathBuf },
	#[error("nsx could not download its {tool}: {cause}.")]
	Unfetched { tool: &'static str, cause: String },
	#[error(
		"The {tool} nsx downloaded is not the one it pins: its SHA-256 differs, and nsx will not run it."
	)]
	Unpinned { tool: &'static str },
	#[error(
		"nsx could not keep its {tool} in the project's private state: {cause}."
	)]
	Unkept {
		tool: &'static str,
		cause: std::io::Error,
	},
	#[error(
		"nsx found no container runtime on this Mac: run nsx init in a terminal and answer yes, and nsx installs one."
	)]
	RuntimelessMac,
	#[error(
		"nsx found no Docker on this machine: install it with `curl -fsSL https://get.docker.com | sh`, then run again."
	)]
	RuntimelessLinux,
	#[error(
		"Docker is installed and not running: start it with `sudo systemctl start docker`, then run again."
	)]
	Stopped,
	#[error("nsx started Docker and it did not answer within three minutes.")]
	Asleep,
	#[error(transparent)]
	Program(#[from] crate::process::error::Error),
	#[error(transparent)]
	File(#[from] crate::files::error::Error),
}
