//! Where the project keeps the cluster's kubeconfig
//!
//! One function: it gives back the path of the kubeconfig nsx reaches the
//! shared cluster with, in the project's private state, as kind and kubectl
//! are given it.
//!
//! ## Reads and writes
//!
//! - names `.langdrift/agent/state/cluster/kubeconfig`, which kind writes
//!
//! ## Rules
//!
//! - The file is kind's to write and kubectl's to read: nsx never reads it,
//!   and it holds nothing but the local cluster's own access.
//! - Outside a project there is no such file: a stop.
//! - A path that is not UTF-8 cannot be given to the tools as it is: a
//!   stop, never another path in its place.

use crate::files::{inside::inside, layout::Layout, root::Root};
use crate::local::{error::Error, kind::Kind};

impl Kind<'_> {
	pub fn kubeconfig(&self) -> Result<String, Error> {
		let root = Root::find(&self.directory)?;
		let file = inside(&root, &Layout::under(&root).kubeconfig)?;
		let spoken = file.to_str().map(String::from);

		spoken.ok_or(Error::Unspoken {
			path: file,
		})
	}
}
