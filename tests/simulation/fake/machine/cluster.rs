//! The five answers of the cluster that keeps what it holds

use nsx::local::{
	cluster::{Asked, Cluster},
	deployment::Deployment,
};

use crate::simulation::fake::machine::Machine;

impl Cluster for Machine {
	fn open<'run>(&'run self, space: &'run str) -> Asked<'run, ()> {
		Box::pin(async move {
			self.asked.borrow_mut().push(format!("open {space}"));
			self.reach("kind get")?;
			self
				.spaces
				.borrow_mut()
				.entry(String::from(space))
				.or_default();

			Ok(())
		})
	}

	fn apply<'run>(
		&'run self,
		space: &'run str,
		deployment: &'run Deployment,
	) -> Asked<'run, ()> {
		Box::pin(async move {
			let name = deployment.name.as_str();

			self
				.asked
				.borrow_mut()
				.push(format!("apply {space} {name}"));
			self.reach("kind get")?;
			self
				.spaces
				.borrow_mut()
				.entry(String::from(space))
				.or_default()
				.push(String::from(name));

			if self.failing == Some(name) {
				return Err(Machine::refusal("kubectl rollout"));
			}

			Ok(())
		})
	}

	fn delete<'run>(
		&'run self,
		space: &'run str,
		deployment: &'run str,
	) -> Asked<'run, ()> {
		Box::pin(async move {
			self
				.asked
				.borrow_mut()
				.push(format!("delete {space} {deployment}"));
			self.reach("kind get")?;

			if self.stuck == Some(deployment) {
				return Err(Machine::refusal("kubectl delete"));
			}

			if let Some(held) = self.spaces.borrow_mut().get_mut(space) {
				held.retain(|name| name != deployment);
			}

			Ok(())
		})
	}

	fn close<'run>(&'run self, space: &'run str) -> Asked<'run, ()> {
		Box::pin(async move {
			self.asked.borrow_mut().push(format!("close {space}"));
			self.reach("kind get")?;

			if self.stalling {
				std::future::pending::<()>().await;
			}

			if !self.clinging {
				self.spaces.borrow_mut().remove(space);
			}

			Ok(())
		})
	}

	fn absent<'run>(&'run self, space: &'run str) -> Asked<'run, bool> {
		Box::pin(async move {
			self.asked.borrow_mut().push(format!("absent {space}"));
			self.reach("kind get")?;

			Ok(!self.spaces.borrow().contains_key(space))
		})
	}
}
