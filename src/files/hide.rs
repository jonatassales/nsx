//! Keeping the private state out of Git
//!
//! One function: it takes the project's root and the paths under
//! `.langdrift/`, and writes the ignore file of the private state, so that
//! Git tracks nothing under `.langdrift/agent/state/`.
//!
//! ## Reads and writes
//!
//! - writes `.langdrift/agent/state/.gitignore`
//!
//! ## Rules
//!
//! - It is written before anything private is: a secret, a call's log, an
//!   execution ledger or the cluster's kubeconfig.
//! - The file ignores everything beside it, itself included.

use crate::files::{
	error::Error, inside::inside, layout::Layout, root::Root, sealed::sealed,
};

pub fn hide(root: &Root, layout: &Layout) -> Result<(), Error> {
	sealed(&inside(root, &layout.ignored)?, b"*\n")
}
