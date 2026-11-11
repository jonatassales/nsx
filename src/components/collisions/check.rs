//! Cross-checking the components
//!
//! One function: it takes the resolved components and gives back the
//! cross-check's result.
//!
//! ## Rules
//!
//! - Any collision of path, port or image stops the plan before the
//!   `crossCheck` line, under the stop rule. No draft is written, so a
//!   colliding draft never reaches the team.
//! - The stop's sentence names every collision: each shared path, then each
//!   shared port, then each shared image, with the components that share it.
//! - Two components share a path when their paths name the same folder on
//!   the disk, however each is spelled: `services/api` and `services/API` on
//!   a disk that ignores case are one folder.
//! - A component whose image is not resolved shares no image and is not
//!   counted among the images resolved.
//!
//! Spec 0079.

use std::collections::BTreeMap;

use crate::components::{
	collisions::Collisions, component::Component, error::Error,
};

impl Collisions {
	pub fn check(components: &[Component]) -> Result<Self, Error> {
		// How many values the components give, and one sentence for each value
		// two or more of them give.
		fn shared<'plan, Value: Ord>(
			components: &'plan [Component],
			value: impl Fn(&'plan Component) -> Option<Value>,
			say: impl Fn(&str, &Value) -> String,
		) -> (usize, Vec<String>) {
			let mut holders = BTreeMap::<Value, Vec<&str>>::new();

			for component in components {
				let Some(held) = value(component) else {
					continue;
				};

				holders
					.entry(held)
					.or_default()
					.push(component.path.as_str());
			}

			let collisions = holders
				.iter()
				.filter(|(_value, paths)| paths.len() > 1)
				.map(|(value, paths)| say(&paths.join(" and "), value))
				.collect();

			(holders.len(), collisions)
		}

		let (_folders, same_folder) = shared(
			components,
			|component| Some(component.folder.as_path()),
			|paths, _folder| format!("{paths} are the same folder"),
		);
		let (ports, same_port) = shared(
			components,
			|component| Some(component.port),
			|paths, port| format!("{paths} share the port {port}"),
		);
		let (_images, same_image) = shared(
			components,
			|component| component.image.as_ref().map(|image| image.0.as_str()),
			|paths, image| format!("{paths} share the image {image}"),
		);
		let images = components
			.iter()
			.filter(|component| component.image.is_some())
			.count();
		let collisions = [same_folder, same_port, same_image].concat();

		if !collisions.is_empty() {
			return Err(Error::Collide {
				collisions: collisions.join("; "),
			});
		}

		Ok(Self {
			ports,
			images,
			collisions: collisions.len(),
		})
	}
}
