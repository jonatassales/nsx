//! The suggestion's sizing line
//!
//! One function: it gives back the line that states the sizing basis. No
//! cited episode measured resources when it prints, since a plan that cites
//! one stops at the `episodes` line: the basis is the spec, not history.
//!
//! ## Simulation
//!
//! - `reasonResources`: `console › reasoning · no cited episode measured cpu or memory · sizing follows the spec, not history`
//! - Card: Analysis · MEASURED RESOURCES: this line adds
//!   `sizing follows the spec, not history` beside `none cited` (the field is
//!   spec 0052's)
//!
//! ## Rules
//!
//! - It asks the judge nothing: the conclusion follows from the retrieval.
//! - It comes after the `episodes` line (spec 0052).
//!
//! ## Open
//!
//! - The wording when a cited episode measured resources: the simulation never
//!   shows it, and neither that wording nor sizing from history is built.
//!
//! Spec 0078.

use crate::judgment::suggestion::Suggestion;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Suggestion {
	pub fn resources() -> Line {
		Line {
			id: String::from("reasonResources"),
			kind: Kind::Output,
			shape: Shape::Narrative {
				text: String::from(
					"reasoning · no cited episode measured cpu or memory · sizing follows the spec, not history",
				),
			},
		}
	}
}
