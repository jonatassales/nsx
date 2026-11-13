//! The retrieval's embedding line
//!
//! One function: it gives back the line the plan prints once the ticket's
//! intent is embedded into its one query vector.
//!
//! ## Simulation
//!
//! - `embed`: `console › embedding the ticket intent · 1 query vector`
//!
//! ## Rules
//!
//! - One ticket is one query, and embedding it makes no paid call.

use crate::rag::retrieval::Retrieval;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Retrieval {
	pub fn embedding() -> Line {
		let text = String::from("embedding the ticket intent · 1 query vector");

		Line {
			id: String::from("embed"),
			kind: Kind::Output,
			shape: Shape::Narrative {
				text,
			},
		}
	}
}
