//! The ticket, as the Ticket card shows it
//!
//! One function: it writes the Ticket card's header into the card data, in
//! the shape `plan.json` holds: the id, the number, the slug, the status and
//! the title. The links arrive later, with their line.
//!
//! ## Rules
//!
//! - A number the id does not state is `null`, never an empty text.
//! - The header is handed to the plan's card data before the plan's first
//!   line.

use crate::contract::{cards::Cards, ticket::Ticket as Card};
use crate::request::ticket::Ticket;

impl Ticket {
	pub fn card(&self, cards: &mut Cards) {
		cards.ticket = Some(Card {
			id: self.id.clone(),
			number: self.number.clone(),
			slug: self.slug.clone(),
			status: Some(self.status.clone()),
			title: Some(self.title.clone()),
			links: None,
		});
	}
}
