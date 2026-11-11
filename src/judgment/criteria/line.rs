//! The criteria's line
//!
//! One function: it gives back the line that states what each criterion asks:
//! each criterion's id followed by its statement, joined by ` · `, in the
//! guardrail's table order.
//!
//! ## Simulation
//!
//! - `judgeGuardrail`: `console › judging · FIN-02 estimate before a paid call · FIN-03 a budget before a paid run · FIN-04 size cpu and memory to the load`
//!
//! ## Rules
//!
//! - It comes after the `guardrail` line and before the `bounds` line
//!   (spec 0054).
//!
//! Spec 0078.

use crate::judgment::criteria::Criteria;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Criteria {
	pub fn line(&self) -> Line {
		let stated = self
			.stated
			.iter()
			.map(|criterion| format!(" · {} {}", criterion.id, criterion.statement))
			.collect::<String>();

		Line {
			id: String::from("judgeGuardrail"),
			kind: Kind::Output,
			shape: Shape::Narrative {
				text: format!("judging{stated}"),
			},
		}
	}
}
