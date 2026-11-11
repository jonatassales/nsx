//! The DOCUMENTS of the Analysis card
//!
//! The `documents` section of `plan.json`: what the plan read, row by row,
//! and how many documents that was.
//!
//! Definition only: the `Documents` type. It carries the two counts,
//! `readCount` and `documentsRead`, and one row per document: `prd`,
//! `designDoc`, `spec` and `guardrail`.
//!
//! ## Rules
//!
//! - Each row is absent until its read line prints, and both counts until the
//!   `✓ read` line prints.
//! - `readCount` counts the ticket, the PRD, the design doc and the spec.
//!   `documentsRead` counts the four documents the ticket links.
//! - No other field is allowed.
//!
//! Spec 0077.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::{
	design::Design, guardrail::Guardrail, product::Product, specified::Specified,
};

#[derive(
	Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema,
)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(inline)]
pub struct Documents {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "u64")]
	pub read_count: Option<u64>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "u64")]
	pub documents_read: Option<u64>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Product")]
	pub prd: Option<Product>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Design")]
	pub design_doc: Option<Design>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Specified")]
	pub spec: Option<Specified>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Guardrail")]
	pub guardrail: Option<Guardrail>,
}
