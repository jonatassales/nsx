//! Making the draft
//!
//! One function: it takes the runtime's id, the revision, the judge's
//! suggestion, the resolved components and their sizing, and gives back the
//! draft: the fence filled in, with the sizing beside it.
//!
//! ## Rules
//!
//! - The runtime's id is the ticket's slug.
//! - The runtime-wide cpu and memory are the bound. Each component's are its
//!   proposed share, with one replica.
//! - The model and effort are the judge's suggestion.
//! - The components keep the spec's order.
//! - A component with no image gets an empty one, which validation counts as
//!   an error: nothing is invented in its place.

use crate::components::component::Component;
use crate::contract::{
	component::Component as Fenced,
	fence::{Fence, VERSION},
};
use crate::judgment::suggestion::Suggestion;
use crate::runtime::{draft::Draft, revision::Revision, sizing::Sizing};

impl Draft {
	pub fn of(
		runtime: &str,
		revision: Revision,
		suggestion: &Suggestion,
		components: &[Component],
		sizing: &Sizing,
	) -> Self {
		let fenced = components
			.iter()
			.map(|component| Fenced {
				path: component.path.clone(),
				stack: String::from(component.stack.name),
				role: String::from(component.role.name),
				port: component.port,
				image: component
					.image
					.as_ref()
					.map(|image| image.0.clone())
					.unwrap_or_default(),
				cpu_millis: component.cpu_millis,
				memory_mib: component.memory_mib,
				replicas: 1,
			})
			.collect();

		Self {
			fence: Fence {
				version: VERSION,
				runtime_id: String::from(runtime),
				revision: revision.0,
				cpu_millis: sizing.bound.cpu_millis,
				memory_mib: sizing.bound.memory_mib,
				model: suggestion.model.clone(),
				effort: suggestion.effort.clone(),
				components: fenced,
			},
			sizing: sizing.clone(),
		}
	}
}
