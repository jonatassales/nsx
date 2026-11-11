//! The five answers of the cluster that keeps nothing

use nsx::local::{
	cluster::{Asked, Cluster},
	deployment::Deployment,
	error::Error,
};
use nsx::process::error::Error as Ended;

use crate::simulation::fake::host::Host;

impl Cluster for Host {
	fn open<'run>(&'run self, _space: &'run str) -> Asked<'run, ()> {
		Box::pin(async { Ok(()) })
	}

	fn apply<'run>(
		&'run self,
		_space: &'run str,
		deployment: &'run Deployment,
	) -> Asked<'run, ()> {
		Box::pin(async move {
			if self.failing == Some(deployment.name.as_str()) {
				return Err(Error::Program(Ended::Failed {
					command: String::from("kubectl rollout"),
					status: 1,
				}));
			}

			Ok(())
		})
	}

	fn delete<'run>(
		&'run self,
		_space: &'run str,
		_deployment: &'run str,
	) -> Asked<'run, ()> {
		Box::pin(async { Ok(()) })
	}

	fn close<'run>(&'run self, _space: &'run str) -> Asked<'run, ()> {
		Box::pin(async { Ok(()) })
	}

	fn absent<'run>(&'run self, _space: &'run str) -> Asked<'run, bool> {
		Box::pin(async move { Ok(!self.clinging) })
	}
}
