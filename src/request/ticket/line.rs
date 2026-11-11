//! The ticket's line
//!
//! One function: it gives back the line that names the ticket the plan read,
//! by its id.
//!
//! ## Simulation
//!
//! - `readTicket`: `console › tool call: read 0031-TICKET-book-a-demo-runtime`
//!
//! ## Rules
//!
//! - The line reports a read that already happened: the ticket is resolved and
//!   its header read before the plan's first line.
//!
//! Spec 0078.

use crate::request::ticket::Ticket;
use crate::session::{kind::Kind, line::Line, shape::Shape};

impl Ticket {
	pub fn line(&self) -> Line {
		Line {
			id: String::from("readTicket"),
			kind: Kind::Output,
			shape: Shape::Narrative {
				text: format!("tool call: read {}", self.id),
			},
		}
	}
}
