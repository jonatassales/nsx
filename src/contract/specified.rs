//! The spec's row of DOCUMENTS
//!
//! What the Analysis card shows of the spec the ticket links, set when the
//! spec's read line prints: how many components it names and how many
//! acceptance criteria it lists.
//!
//! Definition only: the `Specified` type. It carries the document's
//! `shortId`, its `file`, and the two counts, `components` and
//! `acceptanceCriteria`.
//!
//! ## Rules
//!
//! - All four are required. The counts are never negative.
//!
//! Spec 0077.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(inline)]
pub struct Specified {
	pub short_id: String,
	pub file: String,
	pub components: u64,
	pub acceptance_criteria: u64,
}
