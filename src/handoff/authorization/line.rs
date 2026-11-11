//! The intent's line
//!
//! One function: it takes the authorized intent and gives back the line that
//! shows what the run applies before money moves: the model, the effort and
//! the `runtime.md` revision.
//!
//! ## Simulation
//!
//! - `intent`: `console › intent     claude-sonnet-5-5 · effort medium · runtime.md revision 1`
//!
//! ## Rules
//!
//! - The model and the effort are the authorized intent's, so the line
//!   differs between runs. With Claude Opus 5.5 at high effort it reads
//!   `console › intent     claude-opus-5-5 · effort high · runtime.md revision 1`.
//!
//! Spec 0079.

use crate::handoff::authorization::Authorization;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Authorization {
	pub fn line(&self) -> Line {
		Line {
			id: String::from("intent"),
			kind: Kind::Output,
			shape: Shape::Labeled {
				label: String::from("intent"),
				value: format!(
					"{} · effort {} · runtime.md revision {}",
					self.intent.model, self.intent.effort, self.intent.revision
				),
			},
		}
	}
}
