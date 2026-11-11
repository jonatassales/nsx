//! kind, as the cluster the run asks
//!
//! `Kind` is `Cluster` as kind, kubectl and Docker serve it: each operation
//! of the trait is the file of the same name in this folder.
//!
//! Spec 0081.

use crate::local::{
	cluster::{Asked, Cluster},
	deployment::Deployment,
	kind::Kind,
};

impl Cluster for Kind<'_> {
	fn open<'run>(&'run self, space: &'run str) -> Asked<'run, ()> {
		Box::pin(Kind::open(self, space))
	}

	fn apply<'run>(
		&'run self,
		space: &'run str,
		deployment: &'run Deployment,
	) -> Asked<'run, ()> {
		Box::pin(Kind::apply(self, space, deployment))
	}

	fn delete<'run>(
		&'run self,
		space: &'run str,
		deployment: &'run str,
	) -> Asked<'run, ()> {
		Box::pin(Kind::delete(self, space, deployment))
	}

	fn close<'run>(&'run self, space: &'run str) -> Asked<'run, ()> {
		Box::pin(Kind::close(self, space))
	}

	fn absent<'run>(&'run self, space: &'run str) -> Asked<'run, bool> {
		Box::pin(Kind::absent(self, space))
	}
}
