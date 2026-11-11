//! The retrieval's candidates line
//!
//! One function: it gives back the line that counts the candidates and gives
//! the highest and the lowest similarity among them.
//!
//! ## Simulation
//!
//! - `candidates`: `console › 6 candidates · similarity 0.81 to 0.42`
//! - Card: Analysis · EPISODES rows: from this line each row carries its id,
//!   its slug and its similarity, with a bar of the similarity
//!
//! ## Rules
//!
//! - Similarities print with two decimals.
//! - The judge's `judgeRelevance` line follows this one (spec 0051).
//!
//! ## Open
//!
//! - Fewer than six episodes: until the owner adds the case to the simulation,
//!   the line keeps its template with the real count. With no candidate it
//!   reads `0 candidates`, without the similarity part: a wording the
//!   simulation does not show yet, and the owner may replace it.
//!
//! Spec 0078.

use crate::rag::retrieval::Retrieval;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Retrieval {
	pub fn candidates(&self) -> Line {
		let similarities =
			self.candidates.iter().map(|candidate| candidate.similarity);
		let highest = similarities.clone().reduce(f64::max);
		let lowest = similarities.reduce(f64::min);
		let text = match (highest, lowest) {
			(Some(highest), Some(lowest)) => format!(
				"{} candidates · similarity {highest:.2} to {lowest:.2}",
				self.total
			),
			_ => String::from("0 candidates"),
		};

		Line {
			id: String::from("candidates"),
			kind: Kind::Output,
			shape: Shape::Narrative {
				text,
			},
		}
	}
}
