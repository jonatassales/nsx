//! The report to the console
//!
//! What nsx sends the console each time it writes a hand-off record: one call
//! of the tool `langdrift.runtime.report`, carrying the records written so far
//! and where the runtime stands. The records stay files, and the report
//! carries them as written. The console answers with the runtime's id.
//!
//! Definition only: the `Report` type. It carries the `runtimeId`, the
//! `status`, and `plan`, `intent` and `run`: `plan.json`, `intent.json` and
//! `run.json` as last written. The console's answer is `Reply`, in its own
//! file. The pattern of a runtime's id sits beside the type.
//!
//! ## Rules
//!
//! - `status` is required.
//! - `runtimeId` is the console's id of the runtime, a lowercase UUIDv7. It
//!   is absent on a runtime's first report: the console creates it and
//!   answers it, and nsx sends it with every later report.
//! - Each record is absent until nsx has written it. Its schema is referred
//!   to by its `$id`.
//! - A field outside the report or outside a record is refused. So is an
//!   intent with a problem.
//! - The report carries no secret.
//!
//! Spec 0077.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::{cards::Cards, intent::Intent, run::Run, status::Status};

pub const UUID_V7: &str =
	"^[0-9a-f]{8}-[0-9a-f]{4}-7[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$";

// The records keep their own schemas: the report refers to each by its `$id`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(
	title = "Runtime report",
	description = concat!(
		"The arguments of the MCP tool langdrift.runtime.report: the ",
		"runtime's console id, absent on its first report, the run's status ",
		"as nsx states it, and the hand-off records nsx has written so far.",
	),
	extend("$id" = "https://langdrift.md/schemas/report/report.schema.json"),
)]
pub struct Report {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "String", pattern(UUID_V7))]
	pub runtime_id: Option<String>,
	pub status: Status,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Cards")]
	pub plan: Option<Cards>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Intent")]
	pub intent: Option<Intent>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	#[schemars(with = "Run")]
	pub run: Option<Run>,
}
