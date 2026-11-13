//! The relevance's line
//!
//! One function: it gives back the line that counts the candidates that share
//! the ticket's stack and scale, and says why the others differ.
//!
//! ## Simulation
//!
//! - `judgeRelevance`: `console › judging · 3 share the stack and scale · 3 differ: other runtime, no broker, docs only`
//!
//! ## Rules
//!
//! - Each drop reason is listed once, in the set's order: `other runtime`,
//!   `no broker`, `docs only`, joined by `, `.
//! - It comes between the `candidates` line and the `drop` line.
//!
//! ## Open
//!
//! - What the line reads when no candidate is dropped, or none is kept: the
//!   simulation shows one template, with both counts.

use crate::contract::episode::REASONS;
use crate::judgment::relevance::Relevance;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Relevance {
	pub fn line(&self) -> Line {
		let [shared, differing @ ..] = REASONS;
		let kept = self
			.judged
			.iter()
			.filter(|episode| episode.reason == shared)
			.count();
		let reasons = differing
			.into_iter()
			.filter(|reason| {
				self.judged.iter().any(|episode| episode.reason == *reason)
			})
			.collect::<Vec<_>>();
		let differ = if reasons.is_empty() {
			String::new()
		} else {
			format!(": {}", reasons.join(", "))
		};

		Line {
			id: String::from("judgeRelevance"),
			kind: Kind::Output,
			shape: Shape::Narrative {
				text: format!(
					"judging · {kept} share the stack and scale · {} differ{differ}",
					self.judged.len() - kept
				),
			},
		}
	}
}
