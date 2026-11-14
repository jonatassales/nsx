//! The lines of a plan without a key
//!
//! One function: it gives back the two lines the plan prints in place of
//! the judge's first line.
//!
//! ## Simulation
//!
//! - `judgeStopped`: `✗ judge      no Anthropic key · add one on the console's Settings → AI Models`
//! - `planStopped`: `# plan stopped · no draft written · nothing was paid for`

use crate::judgment::keyless::Keyless;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Keyless {
	pub fn lines(&self) -> [Line; 2] {
		[
			Line {
				id: String::from("judgeStopped"),
				kind: Kind::Error,
				shape: Shape::Failure {
					label: String::from("judge"),
					value: String::from(
						"no Anthropic key · add one on the console's Settings → AI Models",
					),
				},
			},
			Line::comment(
				"planStopped",
				"plan stopped · no draft written · nothing was paid for",
			),
		]
	}
}
