//! The Deployment of a component
//!
//! One function: it takes a component of the applied `runtime.md` revision
//! and gives back the Deployment that runs it.
//!
//! ## Rules
//!
//! - The name is the component's path with `/` replaced by `-`.
//! - A path whose name is no lowercase label cannot name a Deployment: the
//!   run stops before anything is created.

use crate::contract::{component::Component, label::label};
use crate::local::{deployment::Deployment, error::Error};

impl Deployment {
	pub fn of(component: &Component) -> Result<Self, Error> {
		let name = component.path.replace('/', "-");

		if !label(&name) {
			return Err(Error::Unnamed {
				path: component.path.clone(),
			});
		}

		Ok(Self {
			name,
			image: component.image.clone(),
			port: component.port,
			cpu_millis: component.cpu_millis,
			memory_mib: component.memory_mib,
			replicas: component.replicas,
		})
	}
}
