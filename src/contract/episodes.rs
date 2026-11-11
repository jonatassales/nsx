//! The episodes answer
//!
//! The judge's answer to its second question: for each episode the search
//! retrieved, whether it is kept or dropped, and why. It feeds the
//! `judgeRelevance` line and the EPISODES rows of the Analysis card.
//!
//! Definition only: the `Episodes` type. It carries `episodes`: one entry per
//! candidate, each with its `id`, its `verdict` and its `reason`.
//!
//! ## Rules
//!
//! - `id` has the form `ep-NNNN`, and each episode is judged once.
//! - `verdict` is `kept` or `dropped`.
//! - `reason` comes from a closed set and fits the verdict: `shares the stack
//!   and scale` when kept; `other runtime`, `no broker` or `docs only` when
//!   dropped.
//! - No other field is allowed.
//! - That a verdict equals the threshold's is checked where the question is
//!   asked, in `judgment` (spec 0051).
//!
//! Spec 0077.

pub mod check;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contract::episode::Episode;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(
	title = "Episodes answer",
	description = concat!(
		"The judge's verdict on every retrieved episode: its ep-NNNN id, ",
		"kept or dropped, and a reason from a closed set, as the EPISODES ",
		"rows show them. A kept episode shares the stack and scale; a ",
		"dropped one runs another runtime, has no broker or is docs only. ",
		"That the reason matches the verdict and that no id repeats are ",
		"rules the Go check enforces.",
	),
	extend("$id" = "https://langdrift.md/schemas/judgment/episodes.answer.schema.json"),
)]
pub struct Episodes {
	pub episodes: Vec<Episode>,
}
