//! The cross-check of the components
//!
//! Once every component is resolved, the plan cross-checks them: the ports are
//! distinct, every image is resolved, and no two components share a path, a
//! port or an image. That makes "no collisions" a checked fact.
//!
//! Definition only: the `Collisions` type: the cross-check's result, as the
//! number of distinct ports, the number of images resolved and the number of
//! collisions. Its functions are the files in `collisions/`.
//!
//! ## Rules
//!
//! - "Images resolved" counts images actually resolved, not components.
//! - Components are unique by path. The cross-check extends that rule to
//!   ports and images.

pub mod card;
pub mod check;
pub mod line;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Collisions {
	pub ports: usize,
	pub images: usize,
	pub collisions: usize,
}
