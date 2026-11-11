//! One linked artifact of the Ticket card
//!
//! A document the ticket links, as LINKED ARTIFACTS shows it.
//!
//! Definition only: the `Linked` type. It carries the document's `kind`, its
//! `file`, its `shortId` and its project-relative `path`, and beside it the
//! four kinds.
//!
//! ## Rules
//!
//! - `kind` is `prd`, `designDoc`, `spec` or `guardrail`, and the four come in
//!   that order.
//! - All four fields are required. No other field is allowed.
//!
//! Spec 0077.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const KINDS: [&str; 4] = ["prd", "designDoc", "spec", "guardrail"];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(inline)]
pub struct Linked {
	#[schemars(extend("enum" = KINDS))]
	pub kind: String,
	pub file: String,
	pub short_id: String,
	pub path: String,
}
