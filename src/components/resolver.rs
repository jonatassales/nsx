//! The image resolver
//!
//! What gives a component its image. It sits behind a trait so that a test
//! can put a resolver in its place that answers what the test needs.
//!
//! Definition only: the `Resolver` trait: one question, a component of a
//! runtime at a revision for its image reference.
//!
//! ## Rules
//!
//! - A resolver builds nothing, loads nothing and starts nothing.
//! - The plan asks through `Image::resolve`, which checks the answer. Nothing
//!   else calls a resolver.

use crate::components::{component::Component, error::Error, image::Image};

pub trait Resolver {
	fn resolve(
		&self,
		runtime: &str,
		revision: i64,
		component: &Component,
	) -> Result<Image, Error>;
}
