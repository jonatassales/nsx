//! The tickets report
//!
//! What the `to-tickets` skill writes, at the path `LANGDRIFT_REPORT` names,
//! when the Tickets stage ends. The run prints its `toTickets` line from these
//! figures.
//!
//! Definition only: the `Tickets` type. It carries `tickets`, a count, and
//! `blockingEdgesRecorded`, a flag.
//!
//! ## Rules
//!
//! - Both fields are required and never `null`. No other field is allowed.
//! - The count is never negative.
//! - Whether a `false` flag fails the stage is `skills`' check (spec 0059).
//!
//! Spec 0077.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(
	title = "Tickets report",
	description = concat!(
		"The to-tickets skill's report, for the stage line \"console › ",
		"to-tickets 3 tickets · blocking edges recorded\".",
	),
	extend("$id" = "https://langdrift.md/schemas/stages/tickets.report.schema.json"),
)]
pub struct Tickets {
	pub tickets: u64,
	pub blocking_edges_recorded: bool,
}
