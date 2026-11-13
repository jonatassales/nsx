//! What the plan act hands on
//!
//! What the end of the plan needs from the act that resolved and sized the
//! components.
//!
//! Definition only: the `Resolved` type: the inventory, its sizing and the
//! revision the draft takes.

use crate::components::inventory::Inventory;
use crate::runtime::{revision::Revision, sizing::Sizing};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resolved {
	pub inventory: Inventory,
	pub sizing: Sizing,
	pub revision: Revision,
}
