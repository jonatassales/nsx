//! The retrieval's opening line
//!
//! One function: it gives back the line the plan prints when the search over
//! the project's episodes begins. The line says what the retrieval is for: a
//! model and an effort for this ticket.
//!
//! ## Simulation
//!
//! - `memory`: `console › RAG over project episodes · a model and effort for this ticket`
//!
//! ## Rules
//!
//! - It follows the plan's `budget` line.

use crate::rag::retrieval::Retrieval;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Retrieval {
	pub fn memory() -> Line {
		let text = String::from(
			"RAG over project episodes · a model and effort for this ticket",
		);

		Line {
			id: String::from("memory"),
			kind: Kind::Output,
			shape: Shape::Narrative {
				text,
			},
		}
	}
}
