//! The console's reply to a report
//!
//! What the console answers each report with: the runtime's id, created on
//! its first report. nsx keeps it and sends it with every later report.
//!
//! Definition only: the `Reply` type. It carries the `runtimeId` and, when
//! the team asked something nsx has not applied yet, `requested`.
//!
//! ## Rules
//!
//! - `runtimeId` is required: a lowercase UUIDv7.
//! - `requested` is absent when nothing is pending (spec 0076).
//! - No other field is allowed.
//!
//! Spec 0077.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::{report::UUID_V7, requested::Requested};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(
	title = "Runtime report answer",
	description = concat!(
		"The result of the MCP tool langdrift.runtime.report: the runtime's ",
		"console id, created on its first report, and the team's request nsx ",
		"has not applied yet, when there is one.",
	),
	extend("$id" = "https://langdrift.md/schemas/report/answer.schema.json"),
)]
pub struct Reply {
	#[schemars(pattern(UUID_V7))]
	pub runtime_id: String,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Requested")]
	pub requested: Option<Requested>,
}
