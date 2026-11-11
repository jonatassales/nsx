//! The PRD's row of DOCUMENTS
//!
//! What the Analysis card shows of the PRD the ticket links, set when the
//! PRD's read line prints.
//!
//! Definition only: the `Product` type. It carries the document's `shortId`,
//! its `file` and its `useCase`.
//!
//! ## Rules
//!
//! - All three are required. No other field is allowed.
//!
//! Spec 0077.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(inline)]
pub struct Product {
	pub short_id: String,
	pub file: String,
	pub use_case: String,
}
