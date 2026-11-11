//! The suggestion's model line
//!
//! One function: it gives back the line that weighs the models the judge did
//! not pick: the suggestion's model reason, as the judge gave it.
//!
//! ## Simulation
//!
//! - `weighModel`: `console › weighing model · Haiku 5.5 retries the broker wiring · Opus 5.5 costs more per outcome`
//!
//! Spec 0078.

use crate::judgment::suggestion::Suggestion;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Suggestion {
	pub fn model(&self) -> Line {
		Line {
			id: String::from("weighModel"),
			kind: Kind::Output,
			shape: Shape::Narrative {
				text: format!("weighing model · {}", self.model_reason),
			},
		}
	}
}
