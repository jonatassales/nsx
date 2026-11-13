//! The judge's verdict on one episode
//!
//! An entry of the judge's episodes answer: one candidate the vector search
//! retrieved, kept or dropped, and why, as an EPISODES row shows it.
//!
//! Definition only: the `Episode` type. It carries its `id`, its `verdict`
//! and its `reason`. The closed sets of verdicts and reasons sit beside it.
//!
//! ## Rules
//!
//! - `id` has the form `ep-NNNN`.
//! - `verdict` is `kept` or `dropped`.
//! - `reason` comes from a closed set: `shares the stack and scale` when
//!   kept; `other runtime`, `no broker` or `docs only` when dropped. That it
//!   fits the verdict is the answer's check.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const VERDICTS: [&str; 2] = ["kept", "dropped"];

pub const REASONS: [&str; 4] = [
	"shares the stack and scale",
	"other runtime",
	"no broker",
	"docs only",
];

// The verdict and the reason are text, so that the answer's check can name
// a value outside its set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
#[schemars(inline)]
pub struct Episode {
	#[schemars(pattern(r"^ep-[0-9]{4}$"))]
	pub id: String,
	#[schemars(extend("enum" = VERDICTS))]
	pub verdict: String,
	#[schemars(extend("enum" = REASONS))]
	pub reason: String,
}
