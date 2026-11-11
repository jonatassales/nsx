//! The suggestion's effort line
//!
//! One function: it gives back the line that weighs the efforts the judge did
//! not pick: the suggestion's effort reason, as the judge gave it.
//!
//! ## Simulation
//!
//! - `weighEffort`: `console › weighing effort · low skips the broker wiring · high is not warranted by 5 components`
//!
//! Spec 0078.

use crate::judgment::suggestion::Suggestion;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Suggestion {
	pub fn effort(&self) -> Line {
		Line {
			id: String::from("weighEffort"),
			kind: Kind::Output,
			shape: Shape::Narrative {
				text: format!("weighing effort · {}", self.effort_reason),
			},
		}
	}
}
