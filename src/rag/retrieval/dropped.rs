//! The retrieval's dropped line
//!
//! One function: it gives back the line that says how many candidates fell
//! below the threshold and how many were kept.
//!
//! ## Simulation
//!
//! - `drop`: `console › dropped 3 below 0.55 · kept 3`
//! - Card: Analysis · EPISODES rows: from this line each row also carries its
//!   verdict, as in `ep-0351 signup-redesign-preview 0.66 kept` and
//!   `ep-0240 batch-export-worker 0.47 dropped · other runtime`
//!
//! ## Rules
//!
//! - It comes after the judge's `judgeRelevance` line and is never
//!   contradicted by it: the judge's verdicts equal the threshold's
//!   (spec 0051).
//! - The kept and total counts also feed the Analysis header's second state,
//!   which is spec 0051's.
//!
//! Spec 0078.

use crate::rag::retrieval::Retrieval;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Retrieval {
	pub fn dropped(&self) -> Line {
		let text = format!(
			"dropped {} below {:.2} · kept {}",
			self.dropped, self.threshold.0, self.kept
		);

		Line {
			id: String::from("drop"),
			kind: Kind::Output,
			shape: Shape::Narrative {
				text,
			},
		}
	}
}
