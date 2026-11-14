//! What the plan has read before its first line
//!
//! The plan reads the ticket, its links, the design doc, the price list and
//! the team's offer before it prints anything, so that the Ticket card and
//! the Workflow card are filled when the panel appears. Every act of the
//! plan works from this one reading.
//!
//! Definition only: the `Brief` type: the project's root and the paths under
//! `.langdrift/`, the ticket and the text of its file, its four links, the
//! design doc, the price list, the candidates the team offers and the
//! workflow.

use crate::files::{layout::Layout, root::Root};
use crate::flow::workflow::Workflow;
use crate::money::{candidates::Candidates, price::Price};
use crate::request::{design::Design, links::Links, ticket::Ticket};

#[derive(Debug, Clone)]
pub struct Brief {
	pub root: Root,
	pub layout: Layout,
	pub ticket: Ticket,
	pub written: String,
	pub links: Links,
	pub design: Design,
	pub prices: Price,
	pub candidates: Candidates,
	pub workflow: Workflow,
}
