//! The components the spec names
//!
//! The inventory is the list of components a plan works on: exactly the paths
//! the spec names, in the spec's order, each resolved into a component. The
//! paths are handed over by `request`, each with the port the spec names
//! beside it, if any.
//!
//! Definition only: the `Inventory` type: the resolved components, in the
//! spec's order. The plan adds each one once its row has printed. Its
//! functions are the files in `inventory/`.
//!
//! ## Rules
//!
//! - The list comes from the spec, never from the command line.
//! - The terminal and the card read in the order the spec wrote the paths.
//! - A component that cannot be resolved stops the plan there, under the stop
//!   rule. The components after it are not inspected.

pub mod card;
pub mod line;

use crate::components::component::Component;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Inventory {
	pub components: Vec<Component>,
}
