//! Resolving a component's image
//!
//! One function: it takes the resolver, the runtime's id, the draft's
//! revision and a component, and gives back the component's image
//! reference, or a refusal.
//!
//! ## Reads and writes
//!
//! - asks the image resolver the session carries
//!
//! ## Rules
//!
//! - An image that does not resolve stops the plan before the `crossCheck`
//!   line, under the stop rule. No draft is written.
//! - Resolving builds nothing, loads nothing and starts nothing.
//! - The answer is checked with the fence's own rule for an image
//!   (`contract/reference.rs`): plain text with no space in it. An answer
//!   that is empty or breaks the rule is refused: it goes into the draft.

use crate::components::{
	component::Component, error::Error, image::Image, resolver::Resolver,
};
use crate::contract::reference::reference;

impl Image {
	pub fn resolve(
		resolver: &dyn Resolver,
		runtime: &str,
		revision: i64,
		component: &Component,
	) -> Result<Self, Error> {
		let image = resolver.resolve(runtime, revision, component)?;

		if !reference(&image.0) {
			return Err(Error::Image {
				path: component.path.clone(),
				image: image.0,
			});
		}

		Ok(image)
	}
}
