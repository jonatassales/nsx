//! `intent.json`: the recorded intent
//!
//! What the team is asked to authorize and what a run applies: one ticket, one
//! revision of `runtime.md`, one model, one effort and the stages a person
//! takes part in. The plan writes it as proposed, and the team authorizes it.
//!
//! Definition only: the `Intent` type. It carries the `ticket`, the
//! `runtimeId`, the `revision`, the `model`, the `effort`, `human` and the
//! `status`. `human` is a `Human`, one flag per stage. `status` is `proposed`
//! or `authorized`, the two statuses beside the type. Its function is the file
//! in `intent/`.
//!
//! ## Rules
//!
//! - Every field is required, and no other field is allowed.
//! - `ticket` is the ticket's bare id, and `model` is a price-list id.
//! - nsx records the team's authorization. It never decides it.
//! - The judge's suggestion does not set `model` or `effort`: the plan
//!   proposes the workflow's default choice, and the team may change it.

pub mod validate;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::{fence::EFFORTS, human::Human};

pub const STATUSES: [&str; 2] = ["proposed", "authorized"];

// The effort and the status are text, so that `validate` can name a value
// outside its set instead of the decoding refusing it unnamed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[schemars(
	title = "Recorded intent",
	description = concat!(
		"The recorded intent (intent.json) that nsx agent run applies, for ",
		"\"console › intent     claude-sonnet-5-5 · effort medium · ",
		"runtime.md revision 1\" and the HUMAN IN THE LOOP ticks. Planning ",
		"always has a person in it.",
	),
	extend("$id" = "https://langdrift.md/schemas/handoff/intent.schema.json"),
)]
pub struct Intent {
	#[schemars(length(min = 1))]
	pub ticket: String,
	#[schemars(length(min = 1))]
	pub runtime_id: String,
	#[schemars(range(min = 1))]
	pub revision: i64,
	#[schemars(length(min = 1))]
	pub model: String,
	#[schemars(extend("enum" = EFFORTS))]
	pub effort: String,
	pub human: Human,
	#[schemars(extend("enum" = STATUSES))]
	pub status: String,
}
