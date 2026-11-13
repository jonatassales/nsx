//! The links' line
//!
//! One function: it takes the ticket's status and gives back the line that
//! states it and names the four links by short id.
//!
//! ## Simulation
//!
//! - `links`: `console › ready-for-agent · links 0002-PRD · 0005-DD · 0011-SPEC · guardrail 0001-finops`
//! - Card: Ticket · LINKED ARTIFACTS, filled when this line prints
//!
//! ## Rules
//!
//! - The order is PRD, design doc, spec, guardrail. The guardrail is preceded
//!   by the label `guardrail`.
//! - The line shows short ids. The card shows the full file names.

use crate::request::links::Links;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Links {
	pub fn line(&self, status: &str) -> Line {
		Line {
			id: String::from("links"),
			kind: Kind::Output,
			shape: Shape::Narrative {
				text: format!(
					"{status} · links {} · {} · {} · guardrail {}",
					self.prd.short,
					self.design.short,
					self.spec.short,
					self.guardrail.short
				),
			},
		}
	}
}
