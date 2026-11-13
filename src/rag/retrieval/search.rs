//! The retrieval's search line
//!
//! One function: it gives back the line the plan prints for the search of the
//! index: the top six episodes by cosine similarity.
//!
//! ## Simulation
//!
//! - `vectorSearch`: `console › vector search · project episodes · top 6 by cosine similarity`
//! - Card: Analysis · EPISODES rows: not set until this line, which shows six
//!   placeholder rows

use crate::rag::retrieval::Retrieval;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Retrieval {
	pub fn search() -> Line {
		let text = String::from(
			"vector search · project episodes · top 6 by cosine similarity",
		);

		Line {
			id: String::from("vectorSearch"),
			kind: Kind::Output,
			shape: Shape::Narrative {
				text,
			},
		}
	}
}
