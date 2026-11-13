//! The gate's line
//!
//! One function: it gives back the lines a run prints once the gate is
//! decided: its line and, when the gate paused, the comment that follows it.
//!
//! ## Simulation
//!
//! - `gateOpened`: `console › gate       reserved $11.40 of $25.00 · opened`
//! - `gatePaused`: `console › gate       reservation $40.80 exceeds the $25.00 budget · paused`
//! - `paused`: `# needs a decision: raise the budget or lower the effort · nothing was paid for`

use crate::money::gate::Gate;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Gate {
	pub fn line(&self) -> Vec<Line> {
		let labeled = |id: &str, kind: Kind, value: String| Line {
			id: String::from(id),
			kind,
			shape: Shape::Labeled {
				label: String::from("gate"),
				value,
			},
		};

		match self {
			Self::Opened {
				reserved,
				budget,
			} => vec![labeled(
				"gateOpened",
				Kind::Success,
				format!(
					"reserved {} of {} · opened",
					reserved.format(),
					budget.format()
				),
			)],
			Self::Paused {
				reserved,
				budget,
			} => vec![
				labeled(
					"gatePaused",
					Kind::Warning,
					format!(
						"reservation {} exceeds the {} budget · paused",
						reserved.format(),
						budget.format()
					),
				),
				Line::comment(
					"paused",
					"needs a decision: raise the budget or lower the effort · nothing was paid for",
				),
			],
		}
	}
}
