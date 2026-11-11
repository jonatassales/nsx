//! The stages, as the records spell them
//!
//! The name of a stage of the workflow, wherever a record names one: the
//! `human` flags of `intent.json`, the `stages` of `run.json`, and the stage a
//! report belongs to.
//!
//! Definition only: the `Stage` type and the five stages whose skill writes a
//! report: `planning`, `spec`, `tickets`, `implement` and `review`.
//!
//! ## Rules
//!
//! - A stage is spelled the same in every record and in the stage call.
//! - A stage outside the set has no report: a report named for it is refused.
//!
//! ## Open
//!
//! - The workflow has two more stages, `dev-ticket` and `merge` (spec 0057),
//!   and `run.json` records the dev ticket among its stages. Whether this type
//!   names all seven is settled when spec 0057's fields join.
//!
//! Spec 0077.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Stage {
	Planning,
	Spec,
	Tickets,
	Implement,
	Review,
}
