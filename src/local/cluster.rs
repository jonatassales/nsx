//! The cluster, as a trait
//!
//! The cluster is the outside system that hosts the runtime. nsx asks five
//! things of it: open a space for the run, apply a Deployment there and wait
//! until it is available, delete a Deployment, close the space, and confirm
//! that the space is absent.
//!
//! Definition only: the trait `Cluster` and its five operations, so that
//! tests can replace it with a fake, and `Asked`, the answer each gives.
//!
//! ## Rules
//!
//! - The golden test scripts it: five Deployments available, then removed,
//!   and the space absent.
//! - "Available" is the Deployment's own available condition with its one
//!   replica running. No probe is added.
//! - A space is the run's own boundary in the cluster, named after the run
//!   and its project.
//!   Spec 0062 named four operations for a cluster made per run; with the
//!   shared cluster the owner kept, closing the space is the fifth.
//!
//! Spec 0081.

use std::future::Future;
use std::pin::Pin;

use crate::local::{deployment::Deployment, error::Error};

pub type Asked<'run, Answer> =
	Pin<Box<dyn Future<Output = Result<Answer, Error>> + 'run>>;

pub trait Cluster {
	fn open<'run>(&'run self, space: &'run str) -> Asked<'run, ()>;

	fn apply<'run>(
		&'run self,
		space: &'run str,
		deployment: &'run Deployment,
	) -> Asked<'run, ()>;

	fn delete<'run>(
		&'run self,
		space: &'run str,
		deployment: &'run str,
	) -> Asked<'run, ()>;

	fn close<'run>(&'run self, space: &'run str) -> Asked<'run, ()>;

	fn absent<'run>(&'run self, space: &'run str) -> Asked<'run, bool>;
}
