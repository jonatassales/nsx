//! What a run applies
//!
//! What the acts of a run need from its opening: where the project is, the
//! intent the team authorized, the plan's card data and the run's id.
//!
//! Definition only: the `Applied` type: the project's root and the paths
//! under `.langdrift/`, the authorized intent, the plan's cards, the run's
//! id, the price list and the workflow.

use crate::contract::cards::Cards;
use crate::evidence::id::Id;
use crate::files::{layout::Layout, root::Root};
use crate::flow::workflow::Workflow;
use crate::handoff::authorization::Authorization;
use crate::money::price::Price;

#[derive(Debug, Clone)]
pub struct Applied {
	pub root: Root,
	pub layout: Layout,
	pub authorization: Authorization,
	pub cards: Cards,
	pub id: Id,
	pub prices: Price,
	pub workflow: Workflow,
}
