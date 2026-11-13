//! The suggestion's line
//!
//! One function: it gives back the line that names the suggested model, by its
//! id, and the suggested effort, so that a person can copy them exactly.
//!
//! ## Simulation
//!
//! - `suggest`: `✓ suggests   claude-sonnet-5-5 · effort medium`
//! - Card: Analysis · SUGGESTED MODEL: `Claude Sonnet 5.5`, with
//!   `claude-sonnet-5-5` beneath it
//! - Card: Analysis · SUGGESTED EFFORT: `medium`
//! - Card: Analysis · `recommendation · the team authorizes`
//! - Card: Analysis · header: `Claude Sonnet 5.5 · effort medium`
//!
//! ## Rules
//!
//! - The line prints the model's id. The model's name on the card comes from
//!   the price list.
//! - It comes after the `weighModel` line and before the `planning` comment.

use crate::judgment::suggestion::Suggestion;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Suggestion {
	pub fn line(&self) -> Line {
		Line {
			id: String::from("suggest"),
			kind: Kind::Success,
			shape: Shape::Result {
				label: String::from("suggests"),
				value: format!("{} · effort {}", self.model, self.effort),
			},
		}
	}
}
