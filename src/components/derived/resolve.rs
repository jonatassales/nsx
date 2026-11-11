//! Deriving a component's image
//!
//! One function: it takes the runtime's id, the draft's revision and a
//! component, and gives back the image reference derived from them.
//!
//! ## Rules
//!
//! - Two components whose paths end in the same folder derive the same
//!   image: the cross-check stops the plan for it.
//!
//! Spec 0079.

use crate::components::{
	component::Component, derived::Derived, error::Error, image::Image,
	resolver::Resolver,
};

impl Resolver for Derived {
	fn resolve(
		&self,
		runtime: &str,
		revision: i64,
		component: &Component,
	) -> Result<Image, Error> {
		let folder = component
			.path
			.trim_end_matches('/')
			.rsplit('/')
			.next()
			.unwrap_or(&component.path);

		Ok(Image(format!("{runtime}/{folder}:r{revision}")))
	}
}
