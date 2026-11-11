//! The six shapes of a line
//!
//! Every line of the simulation is laid out in one of six ways, and a shape
//! carries the words that way of laying out needs: a text, a label and its
//! value, or a component's path, stack and role, and share.
//!
//! Definition only: the `Shape` type, with exactly six shapes: comment,
//! narrative, labeled, result, status and component.
//!
//! ## Rules
//!
//! - The shape says how a line is laid out, never what it means: the kind
//!   does that. The opened gate is a labeled line of the success kind.
//! - The layout itself, with its columns, is `line/render.rs`.
//!
//! Spec 0077.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Shape {
	Comment {
		text: String,
	},
	Narrative {
		text: String,
	},
	Labeled {
		label: String,
		value: String,
	},
	Result {
		label: String,
		value: String,
	},
	Status {
		label: String,
		value: String,
	},
	Component {
		path: String,
		stack_role: String,
		share: String,
	},
}
