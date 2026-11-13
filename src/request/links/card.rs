//! The links, as the Ticket card shows them
//!
//! One function: it writes the four linked artifacts into the Ticket card of
//! the card data, in the order PRD, design doc, spec, guardrail, each with its
//! kind, its file, its short id and its path.
//!
//! ## Simulation
//!
//! - Card: Ticket · LINKED ARTIFACTS, filled when the `links` line prints
//!
//! ## Rules
//!
//! - The card shows the full file names; the line shows the short ids.
//! - The header is written before the plan's first line, so the card is
//!   there when its links arrive. With no header there is no card to add
//!   them to.

use crate::contract::{cards::Cards, linked::Linked};
use crate::request::links::Links;

impl Links {
	pub fn card(&self, cards: &mut Cards) {
		let linked =
			[&self.prd, &self.design, &self.spec, &self.guardrail].map(|link| {
				Linked {
					kind: String::from(link.kind.word),
					file: link.name.clone(),
					short_id: link.short.clone(),
					path: link.path.display().to_string(),
				}
			});

		if let Some(header) = cards.ticket.as_mut() {
			header.links = Some(Vec::from(linked));
		}
	}
}
