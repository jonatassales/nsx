//! One component the spec names
//!
//! An item of the spec's `## Components` section: where a component lives, and
//! the port the spec names beside it, if it names one.
//!
//! Definition only: the `Named` type: the component's project-relative path
//! and its port, if the spec gives one.
//!
//! ## Rules
//!
//! - A port the spec does not name is unknown, never zero. `components/`
//!   finds it in the manifest or in the stack's usual port.
//!
//! Spec 0078.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Named {
	pub path: String,
	pub port: Option<i64>,
}
