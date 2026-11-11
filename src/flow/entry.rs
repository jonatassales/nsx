//! One stage of the workflow file
//!
//! A stage as `harness/workflow.json` lists it: what it is called, the skill
//! the card names for it and who acts in it.
//!
//! Definition only: the `Entry` type, which carries the stage's `id`, its
//! `skill`, absent for Dev ticket and Merge, and `person`.
//!
//! ## Rules
//!
//! - A field the file does not define is refused.
//! - How a run plays a stage is `Stage`'s, not this type's.
//!
//! Spec 0080.

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entry {
	pub id: String,
	pub skill: Option<String>,
	pub person: String,
}
