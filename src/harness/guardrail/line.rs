//! The guardrail's line
//!
//! One function: it gives back the line that names the guardrail the plan read
//! and the criteria the plan step answers to.
//!
//! ## Simulation
//!
//! - `guardrail`: `reads      harness/guardrails/0001-finops.gr.md · FIN-02 · FIN-04`
//! - Card: Analysis · the guardrail document row, set when this line prints.
//!   The row lists every criterion and the enforcement; the line lists only
//!   the plan step's criteria.
//!
//! ## Rules
//!
//! - The path is relative to `.langdrift/`.
//! - The ids are those whose `Checked at` includes `plan`, in table order, not
//!   every id found in the file.
//! - This is the guardrail's only line. It has no `tool call: read` line.
//!
//! ## Open
//!
//! - The line lists FIN-02 and FIN-04 and omits FIN-03, while the judge's line
//!   and the card list all three. The spec follows the simulation literally. Is
//!   the omission meant?
//!
//! Spec 0078.

use std::path::Path;

use crate::harness::guardrail::Guardrail;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Guardrail {
	pub fn line(&self) -> Line {
		let shown = self
			.path
			.strip_prefix(".langdrift")
			.unwrap_or(Path::new(&self.path));
		let named = self
			.criteria
			.iter()
			.filter(|criterion| criterion.plan)
			.map(|criterion| format!(" · {}", criterion.id))
			.collect::<String>();

		Line {
			id: String::from("guardrail"),
			kind: Kind::Output,
			shape: Shape::Status {
				label: String::from("reads"),
				value: format!("{}{named}", shown.display()),
			},
		}
	}
}
