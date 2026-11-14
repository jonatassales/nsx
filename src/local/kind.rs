//! The cluster on the person's machine
//!
//! The real cluster is the machine's kind cluster, named `langdrift`: created
//! the first time a run needs it and kept for the next run. Each run gets a
//! namespace of its own in it, named after the run's space and labelled with
//! it. It needs Docker on the machine and brings its own kind and kubectl.
//!
//! Definition only: the `Kind` type: the runner that starts the tools, how
//! nsx downloads its own, the folder they start in, the tools nsx pins,
//! whether they were brought and whether the cluster was reached already,
//! with the cluster's name, its kubectl context, the label a run's things
//! carry, the bound on a Deployment's wait, in
//! seconds, and the variables the tools may see beside them. Its functions
//! are the files in `kind/`.
//!
//! ## Reads and writes
//!
//! - runs kind, kubectl and Docker
//! - downloads nsx's pinned kind and kubectl into
//!   `.langdrift/agent/state/tools/` (`equip.rs`)
//!
//! ## Rules
//!
//! - Each program nsx starts is given only a small, explicit set of
//!   variables.
//! - Nothing but the run's namespace and its Deployments is created: no
//!   quota, Service, Secret or volume.
//! - The cluster is shared between runs and never deleted by one: the
//!   owner's decision of 2026-11-11.
//! - Only a namespace that carries the run's label is the run's to delete.
//! - The cluster is reached through a kubeconfig of the project's own, in
//!   its private state. The person's kubeconfig, and the context it points
//!   at, are never changed, and `KUBECONFIG` is not passed to the tools.
//! - The cluster is reached once for each `Kind`: the kubeconfig is not
//!   written again before every operation.
//! - An image's name is always given after `--`, so that none is read as a
//!   flag.
//! - nsx's pinned kind and kubectl are brought before the cluster is first
//!   reached, and every tool starts with their folder first on its `PATH`.
//! - A tool that cannot be brought, or a missing Docker, is a stop under the
//!   stop rule.
//! - The wait for a Deployment to be available has a bound of 20 minutes.
//!   Reaching it is a stop with no line of its own.
//!
//! ## Open
//!
//! - An interrupt while kind creates the shared cluster may leave a cluster
//!   that kind lists and that does not work. nsx never deletes the shared
//!   cluster, so until the owner decides otherwise the person removes it
//!   (`kind delete cluster --name langdrift`).
//! - The 20-minute bound is nsx's own choice: the simulation shows no wait.
//! - A namespace per run is the owner's answer to the shared cluster. The
//!   simulation shows only the Deployments.

pub mod absent;
pub mod apply;
pub mod close;
pub mod cluster;
pub mod delete;
pub mod equip;
pub mod kubeconfig;
pub mod kubectl;
pub mod open;
pub mod program;
pub mod reach;
pub mod tools;

use std::cell::Cell;
use std::path::PathBuf;

use crate::local::{download::Download, tool::Tool};
use crate::process::runner::Runner;

pub const CLUSTER: &str = "langdrift";

pub const CONTEXT: &str = "kind-langdrift";

pub const LABEL: &str = "langdrift.md/space";

pub const WAIT: u64 = 1200;

pub const PASSED: [&str; 3] =
	["DOCKER_HOST", "XDG_CONFIG_HOME", "XDG_RUNTIME_DIR"];

pub struct Kind<'run> {
	pub programs: &'run dyn Runner,
	pub downloads: &'run dyn Download,
	pub directory: PathBuf,
	pub pinned: Vec<Tool>,
	pub equipped: Cell<bool>,
	pub reached: Cell<bool>,
}
