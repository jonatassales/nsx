//! The validate line
//!
//! One function: it takes the count the draft's validation gave and gives
//! back the line the plan prints for it.
//!
//! ## Simulation
//!
//! - `validate`: `console › tool call: validate runtime.md draft against the schema · 0 errors`
//! - Card: Plan · VALIDATION, `schema · 0 errors`
//!
//! ## Rules
//!
//! - The line prints only when the count is 0: the simulation shows no other
//!   count.
//! - The plan's `draft` line is not printed here. It belongs to the hand-off
//!   (spec 0050).
//!
//! Spec 0079.

use crate::runtime::draft::Draft;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Draft {
	pub fn line(errors: usize) -> Line {
		Line {
			id: String::from("validate"),
			kind: Kind::Output,
			shape: Shape::Narrative {
				text: format!(
					"tool call: validate runtime.md draft against the schema · {errors} errors"
				),
			},
		}
	}
}
